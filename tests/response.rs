use dash_rs::model::song::{NewgroundsSong, MAIN_SONGS};
use dash_rs::model::creator::Creator;

// This string has been manually edited to ensure that the cases of duplicate and missing songs/creators are also covered by the unit test
const GET_GJ_LEVELS_RESPONSE: &str = "1:62953227:2:Noice:5:1:6:14098234:8:10:9:30:10:329795:12:0:13:21:14:16024:17::43:0:25::18:5:19:24981:42:1:45:30320:3:Tm9pY2UgbGV2ZWwsIGhvcGUgeW91IGxpa2UgaXQ=:15:3:30:0:31:0:37:0:38:0:39:5:46:1:47:2:35:778510|1:63362544:2:Happy Day Gd:5:4:6:14098234:8:10:9:30:10:16162:12:0:13:21:14:2772:17::43:0:25::18:4:19:24979:42:0:45:38476:3:NyB5ZWFycyBvZiBqb3lzIGFuZCBzb3Jyb3dzLCB0aGUgYmVzdCBnYW1lIEkga25ldyBhbmQgSSB3aWxsIHN0YXksIGNvbW1lbnQgaG93IG1hbnkgeWVhcnMgeW91IGhhdmUgYmVlbiBhbmQgd2hhdCBtb3RpdmF0ZWQgeW91IHRvIHN0YXk=:15:3:30:0:31:0:37:3:38:1:39:4:46:1:47:2:35:936243|1:63336521:2:Sound Visualization:5:1:6:4123296:8:10:9:50:10:47521:12:0:13:21:14:2951:17::43:6:25::18:8:19:24979:42:1:45:26229:3:dmlzdWFsIGVmZmVjdHM=:15:3:30:0:31:0:37:3:38:1:39:8:46:1:47:2:35:778510|1:63335504:2:Mind Control:5:2:6:10130943:8:10:9:30:10:51434:12:0:13:21:14:4120:17::43:0:25::18:5:19:24979:42:1:45:49951:3:SXQncyBkb25lIHlhYWF5:15:3:30:0:31:0:37:2:38:1:39:5:46:1:47:2:35:763439|1:63333766:2:An Ode to Time:5:8:6:7226087:8:10:9:10:10:10795:12:0:13:21:14:716:17:1:43:3:25::18:10:19:24979:42:0:45:65535:3:QW5kIGl0IGFsbCBjb21lcyBjcmFzaGluZyBkb3duLiAwOC8xMC8yMC4=:15:4:30:0:31:0:37:3:38:1:39:10:46:1:47:2:35:896364|1:63292359:2:AnnoZone:5:2:6:5897998:8:10:9:50:10:7890:12:0:13:21:14:636:17::43:6:25::18:8:19:24979:42:1:45:51592:3:VGhlIEFubm8gU2VyaWVzIGhhcyByZXR1cm5lZCBhZnRlciAyIHllYXJzIHdpdGggYSAzcmQgbGV2ZWwhIERlZGljYXRlZCB0byB0aGUgQnJveXMuIE1vcmUgQW5ubyBTZXJpZXMgbGV2ZWxzIHRvIGNvbWUuLi4_:15:3:30:0:31:0:37:0:38:1:39:7:46:1:47:2:35:638150|1:63260507:2:Trouble:5:3:6:14221993:8:10:9:50:10:4512:12:0:13:21:14:323:17::43:6:25::18:8:19:24979:42:1:45:58854:3:SSBzaG91bGQgb2Yga25vd24geW91IHdlcmUgdHJvdWJsZS4uLi4uLiAgICAgICAgICAgICBoaQ==:15:3:30:0:31:0:37:1:38:1:39:8:46:1:47:2:35:939885|1:63254272:2:AdrenaLines:5:2:6:116033399:8:10:9:40:10:34848:12:0:13:21:14:2419:17::43:5:25::18:6:19:24979:42:0:45:32956:3:ZGVjbyBsdmw_ISBlbmpveSB1d3U=:15:3:30:0:31:0:37:2:38:1:39:6:46:1:47:2:35:887253|1:63232525:2:Metropolis:5:3:6:1647052:8:10:9:10:10:99385:12:0:13:21:14:5506:17:1:43:3:25::18:10:19:24979:42:1:45:65535:3:V2VsY29tZS4uLg==:15:3:30:0:31:0:37:1:38:1:39:10:46:1:47:2:35:674039|1:61865319:2:Utopia:5:4:6:5570844:8:10:9:50:10:148912:12:0:13:21:14:6961:17::43:6:25::18:8:19:24979:42:0:45:13533:3:dXBkYXRlZCwgc2Vjb25kIHBhcnQgZG9lc250IGxvb2sgc28gdWdseSBub3c=:15:3:30:0:31:0:37:3:38:1:39:8:46:1:47:2:35:761926#1647052:DesTicY:95952|4123296:Cdpre:1478680|5570844:Axils:1341135|7226087:Pauze:1705254|8908442:Nikce:2517174|10130943:FaekI:1727914|14098234:AleXins:4322668|14221993:IFuse:5633975|116033399:KumoriGD:11439344#1~|~638150~|~2~|~-ThunderZone v2-~|~3~|~30~|~4~|~Waterflame~|~5~|~8.78~|~6~|~~|~10~|~http%3A%2F%2Faudio.ngfiles.com%2F638000%2F638150_-ThunderZone-v2-.mp3~|~7~|~UCVuv5iaVR55QXIc_BHQLakA~|~8~|~1~:~1~|~674039~|~2~|~Crystal Tokyo~|~3~|~746~|~4~|~Fantomenk~|~5~|~10.54~|~6~|~~|~10~|~http%3A%2F%2Faudio.ngfiles.com%2F674000%2F674039_Crystal-Tokyo.mp3~|~7~|~UCMSBjXolfz29kxnXpBa7LJA~|~8~|~1~:~1~|~761926~|~2~|~mistmurk + 3MBER - Utopia~|~3~|~49123~|~4~|~mistmurk~|~5~|~7.76~|~6~|~~|~10~|~http%3A%2F%2Faudio.ngfiles.com%2F761000%2F761926_mistmurk--3mber---Utopia.mp3~|~7~|~~|~8~|~1~:~1~|~763439~|~2~|~ColBreakz - Mind Control~|~3~|~47795~|~4~|~ColBreakz~|~5~|~10~|~6~|~~|~10~|~http%3A%2F%2Faudio.ngfiles.com%2F763000%2F763439_ColBreakz---Mind-Control.mp3~|~7~|~~|~8~|~1~:~1~|~778510~|~2~|~Hazmat~|~3~|~23384~|~4~|~CricketSaysChill~|~5~|~1.8~|~6~|~~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F778000%2F778510_Hazmat.mp3%3Ff1512785304~|~7~|~~|~8~|~1~:~1~|~852209~|~2~|~Fried Sushi~|~3~|~28916~|~4~|~lchavasse~|~5~|~5.88~|~6~|~~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F852000%2F852209_Fried-Sushi.mp3%3Ff1552100587~|~7~|~~|~8~|~1~:~1~|~887253~|~2~|~Adrenaline~|~3~|~51089~|~4~|~PsoGnar~|~5~|~10.35~|~6~|~~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F887000%2F887253_Adrenaline.mp3%3Ff1570984144~|~7~|~~|~8~|~1~:~1~|~896364~|~2~|~Beethoven - Moonlight Sonata 3rd Movement (meganeko remix)~|~3~|~48917~|~4~|~meganeko~|~5~|~9.45~|~6~|~~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F896000%2F896364_Beethoven---Moonlight-Sona.mp3%3Ff1575491260~|~7~|~UCP3M2myndqXuAEKKnqm_7SQ~|~8~|~1~:~1~|~936243~|~2~|~Phaera - Ignition~|~3~|~50872~|~4~|~TheArcadium~|~5~|~5.76~|~6~|~~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F936000%2F936243_Phaera---Ignition.mp3%3Ff1590147327~|~7~|~~|~8~|~1#11389:0:10#f687963dcfd37f857633563ee28b0cfadc727c97";

#[test]
fn process_get_gj_levels_response() {
    let levels = dash_rs::response::parse_get_gj_levels_response(GET_GJ_LEVELS_RESPONSE);

    assert!(levels.is_ok(), "{}", levels.unwrap_err());

    let levels = levels.unwrap();

    for level in levels {
        if level.level_id == 63292359 {
            assert!(level.creator.is_none())
        } else {
            assert!(level.creator.is_some())
        }
        if level.level_id == 63260507 {
            assert!(level.custom_song.is_none());
            assert_eq!(level.main_song, Some(MAIN_SONGS[0]))
        } else {
            assert!(level.custom_song.is_some())
        }
    }
}
