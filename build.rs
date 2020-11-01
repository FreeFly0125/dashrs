use serde::Deserialize;
use std::{collections::HashMap, fs, fs::File, io::Write, path::Path};

fn main() {
    generate_boilerplate().unwrap();
}

fn generate_boilerplate() -> std::io::Result<()> {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();

    for entry in fs::read_dir("descriptions")? {
        let entry = entry?;

        if let Ok(file) = entry.file_name().into_string() {
            println!("cargo:rerun-if-changed=descriptions/{}", file);
        }

        let file_stem = entry.path().file_stem().unwrap().to_str().unwrap().to_string();
        let filename = format!("{}.boilerplate", file_stem);

        let dest_path = Path::new(&out_dir).join(filename);

        let mut f = File::create(dest_path)?;

        let ng_yml = fs::read_to_string(entry.path())?;
        let description: ModelDescription = serde_yaml::from_str(&ng_yml).unwrap();

        writeln!(&mut f, "#[allow(non_upper_case_globals, unused_imports)]")?;
        writeln!(&mut f, "const _{}: () = {{", file_stem)?;
        write_preamble(&mut f)?;
        description.write_internal_model(&mut f)?;
        description.write_has_robtop_format_impl(&mut f)?;
        writeln!(&mut f, "}};")?;
    }

    Ok(())
}

fn write_preamble<W: Write>(f: &mut W) -> std::io::Result<()> {
    writeln!(f, "use crate::{{")?;
    writeln!(
        f,
        "serde::{{DeError, HasRobtopFormat, IndexedDeserializer, IndexedSerializer, PercentDecoded, SerError, Thunk, RefThunk, \
         Base64Decoded}},"
    )?;
    writeln!(f, "}};")?;
    writeln!(f, "use serde::{{Deserialize, Serialize}};")?;
    writeln!(f, "use std::{{borrow::{{Cow, Borrow}}, io::Write}};")?;

    Ok(())
}

#[derive(Deserialize, Debug)]
struct ModelDescription {
    r#struct: String,
    map_like: bool,
    separator: String,
    indices: Vec<Index>,

    #[serde(default)]
    special_fields: HashMap<String, String>,
}

impl ModelDescription {
    fn struct_name(&self) -> &str {
        match self.r#struct.split('<').next() {
            Some(first_part) => first_part,
            None => &self.r#struct,
        }
    }

    fn has_any_thunks(&self) -> bool {
        self.indices.iter().any(|idx| idx.thunk)
    }

    pub fn write_internal_model<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        writeln!(f, "#[derive(Serialize, Deserialize)]")?;
        if self.has_any_thunks() {
            writeln!(f, "struct Internal{}<'src, 'bor> {{", self.struct_name())?;
        } else {
            writeln!(f, "struct Internal{}<'src> {{", self.struct_name())?;
        }

        for index in &self.indices {
            index.write_as_field(f)?;
        }

        writeln!(f, "}}")?;

        Ok(())
    }

    pub fn write_has_robtop_format_impl<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        writeln!(f, "impl<'src> HasRobtopFormat<'src> for {} {{", self.r#struct)?;

        writeln!(f, "fn from_robtop_str(input: &'src str) -> Result<Self, DeError> {{")?;
        writeln!(
            f,
            "let internal = Internal{}::deserialize(&mut IndexedDeserializer::new(input, \"{}\", {}))?;",
            self.struct_name(),
            self.separator,
            self.map_like
        )?;

        writeln!(f, "Ok(Self {{")?;

        for index in &self.indices {
            if let Some(ref corresponding_field) = index.maps_to {
                index.generate_from_robtop_conversion(f, corresponding_field)?;
            }
        }

        for (field, code) in &self.special_fields {
            writeln!(f, "{}: {},", field, code)?;
        }

        writeln!(f, "}})")?;
        writeln!(f, "}}")?; // end method

        writeln!(f, "fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {{")?;
        writeln!(f, "let internal = Internal{} {{", self.struct_name())?;

        for index in &self.indices {
            if index.maps_to.is_none() == index.compute.is_none() {
                panic!(
                    "An index can either map 1-to-1 to a dash-rs field, or it can be computed dynamically, not both or neither. Index: {}",
                    index.value
                );
            }

            if let Some(ref mapsto) = index.maps_to {
                index.generate_to_robtop_conversion(f, mapsto)?;
            } else if let Some(ref code) = index.compute {
                writeln!(f, "index_{}: {},", index.value, code)?;
            }
        }

        writeln!(f, "}};")?;

        writeln!(
            f,
            "internal.serialize(&mut IndexedSerializer::new(\"{}\", writer, {}))",
            self.separator, self.map_like
        )?;

        writeln!(f, "}}")?; // end method
        writeln!(f, "}}")?; // end impls

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct Index {
    value: u8,
    r#type: String,

    #[serde(default)]
    thunk: bool,
    #[serde(default)]
    optional: bool, // only relevant for thunks!

    #[serde(default)]
    maps_to: Option<String>,

    #[serde(default)]
    use_into: bool,

    #[serde(default)]
    compute: Option<String>,

    #[serde(default)]
    attributes: Vec<String>,
}

impl Index {
    pub fn write_as_field<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        for attr in &self.attributes {
            writeln!(f, "#[serde({})]", attr)?;
        }
        writeln!(f, "#[serde(rename = \"{}\")]", self.value)?;

        if self.thunk {
            if self.optional {
                writeln!(f, "index_{}: Option<RefThunk<'src, 'bor, {}>>,", self.value, self.r#type)?;
            } else {
                writeln!(f, "index_{}: RefThunk<'src, 'bor, {}>,", self.value, self.r#type)?;
            }
        } else {
            writeln!(f, "index_{}: {},", self.value, self.r#type)?;
        }

        Ok(())
    }

    pub fn generate_from_robtop_conversion<W: Write>(&self, f: &mut W, field_name: &str) -> std::io::Result<()> {
        write!(f, "{}: ", field_name)?;

        if self.thunk {
            if self.optional {
                write!(
                    f,
                    "match internal.index_{} {{None => None, Some(RefThunk::Unprocessed(unproc)) => Some(Thunk::Unprocessed(unproc)), _ \
                     => unreachable!()}}",
                    self.value
                )?;
            } else {
                write!(
                    f,
                    "Thunk::Unprocessed(match internal.index_{} {{RefThunk::Unprocessed(unproc) => unproc, _ => unreachable!() }})",
                    self.value
                )?;
            }
        } else {
            match &self.r#type[..] {
                "&'src str" => write!(f, "Cow::Borrowed(internal.index_{})", self.value)?,
                "Option<&'src str>" => write!(f, "internal.index_{}.map(Cow::Borrowed)", self.value)?,
                _ =>
                    if self.use_into {
                        write!(f, "internal.index_{}.into()", self.value)?
                    } else {
                        write!(f, "internal.index_{}", self.value)?
                    },
            }
        }

        writeln!(f, ",")?;

        Ok(())
    }

    pub fn generate_to_robtop_conversion<W: Write>(&self, f: &mut W, field_name: &str) -> std::io::Result<()> {
        write!(f, "index_{}: ", self.value)?;

        if self.thunk {
            if self.optional {
                write!(f, "self.{}.as_ref().map(|t| t.as_ref_thunk())", field_name)?;
            } else {
                write!(f, "self.{}.as_ref_thunk()", field_name)?;
            }
        } else {
            match &self.r#type[..] {
                "&'src str" => write!(f, "self.{}.as_ref()", field_name)?,
                "Option<&'src str>" => write!(f, "self.{}.as_deref()", field_name)?,
                _ =>
                    if self.use_into {
                        write!(f, "self.{}.into()", field_name)?
                    } else {
                        write!(f, "self.{}", field_name)?
                    },
            }
        }

        writeln!(f, ",")?;

        Ok(())
    }
}
