use num::complex::ComplexFloat;

use crate::square::{Square, self};

pub fn set_bit(bit_board: &mut u64, square: Square, value: bool) {
    debug_assert!(square != Square::None);

    if value {
        *bit_board |= (1u64 << square as u8);
    }
    else { 
        *bit_board &= !(1u64 << square as u8);
    }
}

pub fn get_bit(bit_board: u64, square: Square) -> bool {
    return (bit_board & (1 << square as u8)) != 0
}

pub fn toggle_bit(bit_board: &mut u64, square: Square) {
    *bit_board ^= 1u64 << square as u8;
}

pub fn iterate_set_bits(mut value: u64) -> impl Iterator<Item=u32> {
    //return (0..64).filter(move |x| value >> x & 1 != 0);

    return std::iter::from_fn(move || {
        if value != 0 {
            let index = value.trailing_zeros();
            value ^= 1_u64 << index;
            
            Some(index)
        }
        else {
            None
        }
    });
}

pub fn shift_board(value: u64, dx: i32, dy: i32) -> u64 {
    let index = dx + dy * 8;
    let mask = if dx >= 0 { RIGHT_MOVE_MASK[dx as usize] } else { LEFT_MOVE_MASK[-dx as usize] };
    if index >= 0 {
        return mask & (value << index);
    }
    else{
        return mask & (value >> -index);
    }
}

pub const RIGHT_MOVE_MASK: [u64; 8] = [
    0xffffffffffffffff,
    0xfefefefefefefefe,
    0xfcfcfcfcfcfcfcfc,
    0xf8f8f8f8f8f8f8f8,
    0xf0f0f0f0f0f0f0f0,
    0xe0e0e0e0e0e0e0e0,
    0xc0c0c0c0c0c0c0c0,
    0x8080808080808080,
];

pub const LEFT_MOVE_MASK: [u64; 8] = [
    0xffffffffffffffff,
    0x7f7f7f7f7f7f7f7f,
    0x3f3f3f3f3f3f3f3f,
    0x1f1f1f1f1f1f1f1f,
    0xf0f0f0f0f0f0f0f,
    0x707070707070707,
    0x303030303030303,
    0x101010101010101,
];

pub const WHITE_PASSED_PAWN_MASK: [u64; 64] = [144680345676153344, 361700864190383360, 723401728380766720, 1446803456761533440, 2893606913523066880, 5787213827046133760, 11574427654092267520, 4629771061636907008, 144680345676152832, 361700864190382080, 723401728380764160, 1446803456761528320, 2893606913523056640, 5787213827046113280, 11574427654092226560, 4629771061636890624, 144680345676021760, 361700864190054400, 723401728380108800, 1446803456760217600, 2893606913520435200, 5787213827040870400, 11574427654081740800, 4629771061632696320, 144680345642467328, 361700864106168320, 723401728212336640, 1446803456424673280, 2893606912849346560, 5787213825698693120, 11574427651397386240, 4629771060558954496, 144680337052532736, 361700842631331840, 723401685262663680, 1446803370525327360, 2893606741050654720, 5787213482101309440, 11574426964202618880, 4629770785681047552, 144678138029277184, 361695345073192960, 723390690146385920, 1446781380292771840, 2893562760585543680, 5787125521171087360, 11574251042342174720, 4629700416936869888, 144115188075855872, 360287970189639680, 720575940379279360, 1441151880758558720, 2882303761517117440, 5764607523034234880, 11529215046068469760, 4611686018427387904, 0, 0, 0, 0, 0, 0, 0, 0];
pub const BLACK_PASSED_PAWN_MASK: [u64; 64] = [0, 0, 0, 0, 0, 0, 0, 0, 2, 5, 10, 20, 40, 80, 160, 64, 514, 1285, 2570, 5140, 10280, 20560, 41120, 16448, 131586, 328965, 657930, 1315860, 2631720, 5263440, 10526880, 4210752, 33686018, 84215045, 168430090, 336860180, 673720360, 1347440720, 2694881440, 1077952576, 8623620610, 21559051525, 43118103050, 86236206100, 172472412200, 344944824400, 689889648800, 275955859520, 2207646876162, 5519117190405, 11038234380810, 22076468761620, 44152937523240, 88305875046480, 176611750092960, 70644700037184, 565157600297474, 1412894000743685, 2825788001487370, 5651576002974740, 11303152005949480, 22606304011898960, 45212608023797920, 18085043209519168];

pub const WHITE_QUEEN_CASTLE_MASK: u64 = Square::B1.bit_board() | Square::C1.bit_board() | Square::D1.bit_board();
pub const WHITE_KING_CASTLE_MASK:  u64 = Square::F1.bit_board() | Square::G1.bit_board();
pub const BLACK_QUEEN_CASTLE_MASK: u64 = Square::B8.bit_board() | Square::C8.bit_board() | Square::D8.bit_board();
pub const BLACK_KING_CASTLE_MASK:  u64 = Square::F8.bit_board() | Square::G8.bit_board(); 

//Pawns that attack 2 center squares (e and d pawns)
pub const DOUBLE_PAWN_CENTER_ATTACK_WHITE: u64 = 404226048;
pub const DOUBLE_PAWN_CENTER_ATTACK_BLACK: u64 = 26491358281728;

//Pawns that attack 1 center square (c and f pawns)
pub const PAWN_CENTER_ATTACK_WHITE: u64 = 606339072;
pub const PAWN_CENTER_ATTACK_BLACK: u64 = 39737037422592;

pub const RANK_MASKS: [u64; 8] = [
    0xff, 
    0xff00, 
    0xff0000, 
    0xff000000, 
    0xff00000000, 
    0xff0000000000,
    0xff000000000000,
    0xff00000000000000];

pub const NEIGHBOUR_FILES: [u64; 8] = [144680345676153346, 361700864190383365, 723401728380766730, 1446803456761533460, 2893606913523066920, 5787213827046133840, 11574427654092267680, 4629771061636907072];

pub const WHITE_PAWN_ATTACKS: [u64; 64] = [512, 1280, 2560, 5120, 10240, 20480, 40960, 16384, 131072, 327680, 655360, 1310720, 2621440, 5242880, 10485760, 4194304, 33554432, 83886080, 167772160, 335544320, 671088640, 1342177280, 2684354560, 1073741824, 8589934592, 21474836480, 42949672960, 85899345920, 171798691840, 343597383680, 687194767360, 274877906944, 2199023255552, 5497558138880, 10995116277760, 21990232555520, 43980465111040, 87960930222080, 175921860444160, 70368744177664, 562949953421312, 1407374883553280, 2814749767106560, 5629499534213120, 11258999068426240, 22517998136852480, 45035996273704960, 18014398509481984, 144115188075855872, 360287970189639680, 720575940379279360, 1441151880758558720, 2882303761517117440, 5764607523034234880, 11529215046068469760, 4611686018427387904, 0, 0, 0, 0, 0, 0, 0, 0];
pub const BLACK_PAWN_ATTACKS: [u64; 64] = [0, 0, 0, 0, 0, 0, 0, 0, 2, 5, 10, 20, 40, 80, 160, 64, 512, 1280, 2560, 5120, 10240, 20480, 40960, 16384, 131072, 327680, 655360, 1310720, 2621440, 5242880, 10485760, 4194304, 33554432, 83886080, 167772160, 335544320, 671088640, 1342177280, 2684354560, 1073741824, 8589934592, 21474836480, 42949672960, 85899345920, 171798691840, 343597383680, 687194767360, 274877906944, 2199023255552, 5497558138880, 10995116277760, 21990232555520, 43980465111040, 87960930222080, 175921860444160, 70368744177664, 562949953421312, 1407374883553280, 2814749767106560, 5629499534213120, 11258999068426240, 22517998136852480, 45035996273704960, 18014398509481984];
pub const KNIGHT_ATTACKS: [u64; 64] = [132096, 329728, 659712, 1319424, 2638848, 5277696, 10489856, 4202496, 33816580, 84410376, 168886289, 337772578, 675545156, 1351090312, 2685403152, 1075839008, 8657044482, 21609056261, 43234889994, 86469779988, 172939559976, 345879119952, 687463207072, 275414786112, 2216203387392, 5531918402816, 11068131838464, 22136263676928, 44272527353856, 88545054707712, 175990581010432, 70506185244672, 567348067172352, 1416171111120896, 2833441750646784, 5666883501293568, 11333767002587136, 22667534005174272, 45053588738670592, 18049583422636032, 145241105196122112, 362539804446949376, 725361088165576704, 1450722176331153408, 2901444352662306816, 5802888705324613632, 11533718717099671552, 4620693356194824192, 288234782788157440, 576469569871282176, 1224997833292120064, 2449995666584240128, 4899991333168480256, 9799982666336960512, 1152939783987658752, 2305878468463689728, 1128098930098176, 2257297371824128, 4796069720358912, 9592139440717824, 19184278881435648, 38368557762871296, 4679521487814656, 9077567998918656];
pub const DIAGONAL_ATTACKS: [u64; 64] = [9241421688590303744, 36099303471056128, 141012904249856, 550848566272, 6480472064, 1108177604608, 283691315142656, 72624976668147712, 4620710844295151618, 9241421688590368773, 36099303487963146, 141017232965652, 1659000848424, 283693466779728, 72624976676520096, 145249953336262720, 2310355422147510788, 4620710844311799048, 9241421692918565393, 36100411639206946, 424704217196612, 72625527495610504, 145249955479592976, 290499906664153120, 1155177711057110024, 2310355426409252880, 4620711952330133792, 9241705379636978241, 108724279602332802, 145390965166737412, 290500455356698632, 580999811184992272, 577588851267340304, 1155178802063085600, 2310639079102947392, 4693335752243822976, 9386671504487645697, 326598935265674242, 581140276476643332, 1161999073681608712, 288793334762704928, 577868148797087808, 1227793891648880768, 2455587783297826816, 4911175566595588352, 9822351133174399489, 1197958188344280066, 2323857683139004420, 144117404414255168, 360293502378066048, 720587009051099136, 1441174018118909952, 2882348036221108224, 5764696068147249408, 11529391036782871041, 4611756524879479810, 567382630219904, 1416240237150208, 2833579985862656, 5667164249915392, 11334324221640704, 22667548931719168, 45053622886727936, 18049651735527937];
pub const ORTHOGONAL_ATTACKS: [u64; 64] = [72340172838076926, 144680345676153597, 289360691352306939, 578721382704613623, 1157442765409226991, 2314885530818453727, 4629771061636907199, 9259542123273814143, 72340172838141441, 144680345676217602, 289360691352369924, 578721382704674568, 1157442765409283856, 2314885530818502432, 4629771061636939584, 9259542123273813888, 72340172854657281, 144680345692602882, 289360691368494084, 578721382720276488, 1157442765423841296, 2314885530830970912, 4629771061645230144, 9259542123273748608, 72340177082712321, 144680349887234562, 289360695496279044, 578721386714368008, 1157442769150545936, 2314885534022901792, 4629771063767613504, 9259542123257036928, 72341259464802561, 144681423712944642, 289361752209228804, 578722409201797128, 1157443723186933776, 2314886351157207072, 4629771607097753664, 9259542118978846848, 72618349279904001, 144956323094725122, 289632270724367364, 578984165983651848, 1157687956502220816, 2315095537539358752, 4629910699613634624, 9259541023762186368, 143553341945872641, 215330564830528002, 358885010599838724, 645993902138460168, 1220211685215703056, 2368647251370188832, 4665518383679160384, 9259260648297103488, 18302911464433844481, 18231136449196065282, 18087586418720506884, 17800486357769390088, 17226286235867156496, 16077885992062689312, 13781085504453754944, 9187484529235886208];
pub const QUEEN_ATTACKS: [u64; 64] = [9313761861428380670, 180779649147209725, 289501704256556795, 578721933553179895, 1157442771889699055, 2314886638996058335, 4630054752952049855, 9332167099941961855, 4693051017133293059, 9386102034266586375, 325459994840333070, 578862399937640220, 1157444424410132280, 2315169224285282160, 4702396038313459680, 9404792076610076608, 2382695595002168069, 4765391190004401930, 9530782384287059477, 614821794359483434, 1157867469641037908, 2387511058326581416, 4775021017124823120, 9550042029937901728, 1227517888139822345, 2455035776296487442, 4910072647826412836, 9820426766351346249, 1266167048752878738, 2460276499189639204, 4920271519124312136, 9840541934442029200, 649930110732142865, 1299860225776030242, 2600000831312176196, 5272058161445620104, 10544115227674579473, 2641485286422881314, 5210911883574396996, 10421541192660455560, 361411684042608929, 722824471891812930, 1517426162373248132, 3034571949281478664, 6068863523097809168, 12137446670713758241, 5827868887957914690, 11583398706901190788, 287670746360127809, 575624067208594050, 1079472019650937860, 2087167920257370120, 4102559721436811280, 8133343319517438240, 16194909420462031425, 13871017173176583298, 18303478847064064385, 18232552689433215490, 18090419998706369540, 17806153522019305480, 17237620560088797200, 16100553540994408480, 13826139127340482880, 9205534180971414145];
pub const KING_ATTACKS: [u64; 64] = [770, 1797, 3594, 7188, 14376, 28752, 57504, 49216, 197123, 460039, 920078, 1840156, 3680312, 7360624, 14721248, 12599488, 50463488, 117769984, 235539968, 471079936, 942159872, 1884319744, 3768639488, 3225468928, 12918652928, 30149115904, 60298231808, 120596463616, 241192927232, 482385854464, 964771708928, 825720045568, 3307175149568, 7718173671424, 15436347342848, 30872694685696, 61745389371392, 123490778742784, 246981557485568, 211384331665408, 846636838289408, 1975852459884544, 3951704919769088, 7903409839538176, 15806819679076352, 31613639358152704, 63227278716305408, 54114388906344448, 216739030602088448, 505818229730443264, 1011636459460886528, 2023272918921773056, 4046545837843546112, 8093091675687092224, 16186183351374184448, 13853283560024178688, 144959613005987840, 362258295026614272, 724516590053228544, 1449033180106457088, 2898066360212914176, 5796132720425828352, 11592265440851656704, 4665729213955833856];

pub fn get_in_between(s1: Square, s2: Square) -> u64 {
    return IN_BETWEEN_SQUARES[s1 as usize + (s2 as usize * 64)];
}

const IN_BETWEEN_SQUARES: [u64; 64 * 64] = [
    0, 0, 2, 6, 14, 30, 62, 126, 0, 0, 0, 0, 0, 0, 0, 0, 256, 0, 512, 0, 0, 0, 0, 0, 65792, 0, 0, 262656, 0, 0, 0, 0, 16843008, 0, 0, 0, 134480384, 0, 0, 0, 4311810304, 0, 0, 0, 0, 68853957120, 0, 0, 1103823438080, 0, 0, 0, 0, 0, 35253226045952, 0, 282578800148736, 0, 0, 0, 0, 0, 0, 18049651735527936,
    0, 0, 0, 4, 12, 28, 60, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 512, 0, 1024, 0, 0, 0, 0, 0, 131584, 0, 0, 525312, 0, 0, 0, 0, 33686016, 0, 0, 0, 268960768, 0, 0, 0, 8623620608, 0, 0, 0, 0, 137707914240, 0, 0, 2207646876160, 0, 0, 0, 0, 0, 70506452091904, 0, 565157600297472, 0, 0, 0, 0, 0, 0,
    2, 0, 0, 0, 8, 24, 56, 120, 0, 0, 0, 0, 0, 0, 0, 0, 512, 0, 1024, 0, 2048, 0, 0, 0, 0, 0, 263168, 0, 0, 1050624, 0, 0, 0, 0, 67372032, 0, 0, 0, 537921536, 0, 0, 0, 17247241216, 0, 0, 0, 0, 275415828480, 0, 0, 4415293752320, 0, 0, 0, 0, 0, 0, 0, 1130315200594944, 0, 0, 0, 0, 0,
    6, 4, 0, 0, 0, 16, 48, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 0, 2048, 0, 4096, 0, 0, 132096, 0, 0, 526336, 0, 0, 2101248, 0, 0, 0, 0, 134744064, 0, 0, 0, 1075843072, 0, 0, 0, 34494482432, 0, 0, 0, 0, 0, 0, 0, 8830587504640, 0, 0, 0, 0, 0, 0, 0, 2260630401189888, 0, 0, 0, 0,
    14, 12, 8, 0, 0, 0, 32, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2048, 0, 4096, 0, 8192, 0, 0, 264192, 0, 0, 1052672, 0, 0, 4202496, 33818624, 0, 0, 0, 269488128, 0, 0, 0, 0, 0, 0, 0, 68988964864, 0, 0, 0, 0, 0, 0, 0, 17661175009280, 0, 0, 0, 0, 0, 0, 0, 4521260802379776, 0, 0, 0,
    30, 28, 24, 16, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4096, 0, 8192, 0, 16384, 0, 0, 528384, 0, 0, 2105344, 0, 0, 0, 67637248, 0, 0, 0, 538976256, 0, 0, 8657571840, 0, 0, 0, 0, 137977929728, 0, 0, 0, 0, 0, 0, 0, 35322350018560, 0, 0, 0, 0, 0, 0, 0, 9042521604759552, 0, 0,
    62, 60, 56, 48, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8192, 0, 16384, 0, 0, 0, 0, 1056768, 0, 0, 4210688, 0, 0, 0, 135274496, 0, 0, 0, 1077952512, 0, 0, 17315143680, 0, 0, 0, 0, 275955859456, 0, 2216338399232, 0, 0, 0, 0, 0, 70644700037120, 0, 0, 0, 0, 0, 0, 0, 18085043209519104, 0,
    126, 124, 120, 112, 96, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16384, 0, 32768, 0, 0, 0, 0, 2113536, 0, 0, 8421376, 0, 0, 0, 270548992, 0, 0, 0, 2155905024, 0, 0, 34630287360, 0, 0, 0, 0, 551911718912, 0, 4432676798464, 0, 0, 0, 0, 0, 141289400074240, 567382630219776, 0, 0, 0, 0, 0, 0, 36170086419038208,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 512, 1536, 3584, 7680, 15872, 32256, 0, 0, 0, 0, 0, 0, 0, 0, 65536, 0, 131072, 0, 0, 0, 0, 0, 16842752, 0, 0, 67239936, 0, 0, 0, 0, 4311810048, 0, 0, 0, 34426978304, 0, 0, 0, 1103823437824, 0, 0, 0, 0, 17626613022720, 0, 0, 282578800148480, 0, 0, 0, 0, 0, 9024825867763712, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 3072, 7168, 15360, 31744, 0, 0, 0, 0, 0, 0, 0, 0, 0, 131072, 0, 262144, 0, 0, 0, 0, 0, 33685504, 0, 0, 134479872, 0, 0, 0, 0, 8623620096, 0, 0, 0, 68853956608, 0, 0, 0, 2207646875648, 0, 0, 0, 0, 35253226045440, 0, 0, 565157600296960, 0, 0, 0, 0, 0, 18049651735527424,
    0, 0, 0, 0, 0, 0, 0, 0, 512, 0, 0, 0, 2048, 6144, 14336, 30720, 0, 0, 0, 0, 0, 0, 0, 0, 131072, 0, 262144, 0, 524288, 0, 0, 0, 0, 0, 67371008, 0, 0, 268959744, 0, 0, 0, 0, 17247240192, 0, 0, 0, 137707913216, 0, 0, 0, 4415293751296, 0, 0, 0, 0, 70506452090880, 0, 0, 1130315200593920, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 1536, 1024, 0, 0, 0, 4096, 12288, 28672, 0, 0, 0, 0, 0, 0, 0, 0, 0, 262144, 0, 524288, 0, 1048576, 0, 0, 33816576, 0, 0, 134742016, 0, 0, 537919488, 0, 0, 0, 0, 34494480384, 0, 0, 0, 275415826432, 0, 0, 0, 8830587502592, 0, 0, 0, 0, 0, 0, 0, 2260630401187840, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 3584, 3072, 2048, 0, 0, 0, 8192, 24576, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 524288, 0, 1048576, 0, 2097152, 0, 0, 67633152, 0, 0, 269484032, 0, 0, 1075838976, 8657567744, 0, 0, 0, 68988960768, 0, 0, 0, 0, 0, 0, 0, 17661175005184, 0, 0, 0, 0, 0, 0, 0, 4521260802375680, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 7680, 7168, 6144, 4096, 0, 0, 0, 16384, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1048576, 0, 2097152, 0, 4194304, 0, 0, 135266304, 0, 0, 538968064, 0, 0, 0, 17315135488, 0, 0, 0, 137977921536, 0, 0, 2216338391040, 0, 0, 0, 0, 35322350010368, 0, 0, 0, 0, 0, 0, 0, 9042521604751360, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 15872, 15360, 14336, 12288, 8192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2097152, 0, 4194304, 0, 0, 0, 0, 270532608, 0, 0, 1077936128, 0, 0, 0, 34630270976, 0, 0, 0, 275955843072, 0, 0, 4432676782080, 0, 0, 0, 0, 70644700020736, 0, 567382630203392, 0, 0, 0, 0, 0, 18085043209502720, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 32256, 31744, 30720, 28672, 24576, 16384, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4194304, 0, 8388608, 0, 0, 0, 0, 541065216, 0, 0, 2155872256, 0, 0, 0, 69260541952, 0, 0, 0, 551911686144, 0, 0, 8865353564160, 0, 0, 0, 0, 141289400041472, 0, 1134765260406784, 0, 0, 0, 0, 0, 36170086419005440,
    256, 0, 512, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 131072, 393216, 917504, 1966080, 4063232, 8257536, 0, 0, 0, 0, 0, 0, 0, 0, 16777216, 0, 33554432, 0, 0, 0, 0, 0, 4311744512, 0, 0, 17213423616, 0, 0, 0, 0, 1103823372288, 0, 0, 0, 8813306445824, 0, 0, 0, 282578800082944, 0, 0, 0, 0, 4512412933816320, 0, 0,
    0, 512, 0, 1024, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 262144, 786432, 1835008, 3932160, 8126464, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33554432, 0, 67108864, 0, 0, 0, 0, 0, 8623489024, 0, 0, 34426847232, 0, 0, 0, 0, 2207646744576, 0, 0, 0, 17626612891648, 0, 0, 0, 565157600165888, 0, 0, 0, 0, 9024825867632640, 0,
    512, 0, 1024, 0, 2048, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 131072, 0, 0, 0, 524288, 1572864, 3670016, 7864320, 0, 0, 0, 0, 0, 0, 0, 0, 33554432, 0, 67108864, 0, 134217728, 0, 0, 0, 0, 0, 17246978048, 0, 0, 68853694464, 0, 0, 0, 0, 4415293489152, 0, 0, 0, 35253225783296, 0, 0, 0, 1130315200331776, 0, 0, 0, 0, 18049651735265280,
    0, 1024, 0, 2048, 0, 4096, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 393216, 262144, 0, 0, 0, 1048576, 3145728, 7340032, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67108864, 0, 134217728, 0, 268435456, 0, 0, 8657043456, 0, 0, 34493956096, 0, 0, 137707388928, 0, 0, 0, 0, 8830586978304, 0, 0, 0, 70506451566592, 0, 0, 0, 2260630400663552, 0, 0, 0, 0,
    0, 0, 2048, 0, 4096, 0, 8192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 917504, 786432, 524288, 0, 0, 0, 2097152, 6291456, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134217728, 0, 268435456, 0, 536870912, 0, 0, 17314086912, 0, 0, 68987912192, 0, 0, 275414777856, 2216337342464, 0, 0, 0, 17661173956608, 0, 0, 0, 0, 0, 0, 0, 4521260801327104, 0, 0, 0,
    0, 0, 0, 4096, 0, 8192, 0, 16384, 0, 0, 0, 0, 0, 0, 0, 0, 1966080, 1835008, 1572864, 1048576, 0, 0, 0, 4194304, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 268435456, 0, 536870912, 0, 1073741824, 0, 0, 34628173824, 0, 0, 137975824384, 0, 0, 0, 4432674684928, 0, 0, 0, 35322347913216, 0, 0, 567382628106240, 0, 0, 0, 0, 9042521602654208, 0, 0,
    0, 0, 0, 0, 8192, 0, 16384, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4063232, 3932160, 3670016, 3145728, 2097152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 536870912, 0, 1073741824, 0, 0, 0, 0, 69256347648, 0, 0, 275951648768, 0, 0, 0, 8865349369856, 0, 0, 0, 70644695826432, 0, 0, 1134765256212480, 0, 0, 0, 0, 18085043205308416, 0,
    0, 0, 0, 0, 0, 16384, 0, 32768, 0, 0, 0, 0, 0, 0, 0, 0, 8257536, 8126464, 7864320, 7340032, 6291456, 4194304, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1073741824, 0, 2147483648, 0, 0, 0, 0, 138512695296, 0, 0, 551903297536, 0, 0, 0, 17730698739712, 0, 0, 0, 141289391652864, 0, 0, 2269530512424960, 0, 0, 0, 0, 36170086410616832,
    65792, 0, 0, 132096, 0, 0, 0, 0, 65536, 0, 131072, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33554432, 100663296, 234881024, 503316480, 1040187392, 2113929216, 0, 0, 0, 0, 0, 0, 0, 0, 4294967296, 0, 8589934592, 0, 0, 0, 0, 0, 1103806595072, 0, 0, 4406636445696, 0, 0, 0, 0, 282578783305728, 0, 0, 0, 2256206450130944, 0, 0, 0,
    0, 131584, 0, 0, 264192, 0, 0, 0, 0, 131072, 0, 262144, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67108864, 201326592, 469762048, 1006632960, 2080374784, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8589934592, 0, 17179869184, 0, 0, 0, 0, 0, 2207613190144, 0, 0, 8813272891392, 0, 0, 0, 0, 565157566611456, 0, 0, 0, 4512412900261888, 0, 0,
    0, 0, 263168, 0, 0, 528384, 0, 0, 131072, 0, 262144, 0, 524288, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33554432, 0, 0, 0, 134217728, 402653184, 939524096, 2013265920, 0, 0, 0, 0, 0, 0, 0, 0, 8589934592, 0, 17179869184, 0, 34359738368, 0, 0, 0, 0, 0, 4415226380288, 0, 0, 17626545782784, 0, 0, 0, 0, 1130315133222912, 0, 0, 0, 9024825800523776, 0,
    262656, 0, 0, 526336, 0, 0, 1056768, 0, 0, 262144, 0, 524288, 0, 1048576, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100663296, 67108864, 0, 0, 0, 268435456, 805306368, 1879048192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17179869184, 0, 34359738368, 0, 68719476736, 0, 0, 2216203124736, 0, 0, 8830452760576, 0, 0, 35253091565568, 0, 0, 0, 0, 2260630266445824, 0, 0, 0, 18049651601047552,
    0, 525312, 0, 0, 1052672, 0, 0, 2113536, 0, 0, 524288, 0, 1048576, 0, 2097152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 234881024, 201326592, 134217728, 0, 0, 0, 536870912, 1610612736, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34359738368, 0, 68719476736, 0, 137438953472, 0, 0, 4432406249472, 0, 0, 17660905521152, 0, 0, 70506183131136, 567382359670784, 0, 0, 0, 4521260532891648, 0, 0, 0,
    0, 0, 1050624, 0, 0, 2105344, 0, 0, 0, 0, 0, 1048576, 0, 2097152, 0, 4194304, 0, 0, 0, 0, 0, 0, 0, 0, 503316480, 469762048, 402653184, 268435456, 0, 0, 0, 1073741824, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68719476736, 0, 137438953472, 0, 274877906944, 0, 0, 8864812498944, 0, 0, 35321811042304, 0, 0, 0, 1134764719341568, 0, 0, 0, 9042521065783296, 0, 0,
    0, 0, 0, 2101248, 0, 0, 4210688, 0, 0, 0, 0, 0, 2097152, 0, 4194304, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1040187392, 1006632960, 939524096, 805306368, 536870912, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 137438953472, 0, 274877906944, 0, 0, 0, 0, 17729624997888, 0, 0, 70643622084608, 0, 0, 0, 2269529438683136, 0, 0, 0, 18085042131566592, 0,
    0, 0, 0, 0, 4202496, 0, 0, 8421376, 0, 0, 0, 0, 0, 4194304, 0, 8388608, 0, 0, 0, 0, 0, 0, 0, 0, 2113929216, 2080374784, 2013265920, 1879048192, 1610612736, 1073741824, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 274877906944, 0, 549755813888, 0, 0, 0, 0, 35459249995776, 0, 0, 141287244169216, 0, 0, 0, 4539058877366272, 0, 0, 0, 36170084263133184,
    16843008, 0, 0, 0, 33818624, 0, 0, 0, 16842752, 0, 0, 33816576, 0, 0, 0, 0, 16777216, 0, 33554432, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8589934592, 25769803776, 60129542144, 128849018880, 266287972352, 541165879296, 0, 0, 0, 0, 0, 0, 0, 0, 1099511627776, 0, 2199023255552, 0, 0, 0, 0, 0, 282574488338432, 0, 0, 1128098930098176, 0, 0, 0, 0,
    0, 33686016, 0, 0, 0, 67637248, 0, 0, 0, 33685504, 0, 0, 67633152, 0, 0, 0, 0, 33554432, 0, 67108864, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17179869184, 51539607552, 120259084288, 257698037760, 532575944704, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2199023255552, 0, 4398046511104, 0, 0, 0, 0, 0, 565148976676864, 0, 0, 2256197860196352, 0, 0, 0,
    0, 0, 67372032, 0, 0, 0, 135274496, 0, 0, 0, 67371008, 0, 0, 135266304, 0, 0, 33554432, 0, 67108864, 0, 134217728, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8589934592, 0, 0, 0, 34359738368, 103079215104, 240518168576, 515396075520, 0, 0, 0, 0, 0, 0, 0, 0, 2199023255552, 0, 4398046511104, 0, 8796093022208, 0, 0, 0, 0, 0, 1130297953353728, 0, 0, 4512395720392704, 0, 0,
    0, 0, 0, 134744064, 0, 0, 0, 270548992, 67239936, 0, 0, 134742016, 0, 0, 270532608, 0, 0, 67108864, 0, 134217728, 0, 268435456, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25769803776, 17179869184, 0, 0, 0, 68719476736, 206158430208, 481036337152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4398046511104, 0, 8796093022208, 0, 17592186044416, 0, 0, 567347999932416, 0, 0, 2260595906707456, 0, 0, 9024791440785408, 0,
    134480384, 0, 0, 0, 269488128, 0, 0, 0, 0, 134479872, 0, 0, 269484032, 0, 0, 541065216, 0, 0, 134217728, 0, 268435456, 0, 536870912, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60129542144, 51539607552, 34359738368, 0, 0, 0, 137438953472, 412316860416, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8796093022208, 0, 17592186044416, 0, 35184372088832, 0, 0, 1134695999864832, 0, 0, 4521191813414912, 0, 0, 18049582881570816,
    0, 268960768, 0, 0, 0, 538976256, 0, 0, 0, 0, 268959744, 0, 0, 538968064, 0, 0, 0, 0, 0, 268435456, 0, 536870912, 0, 1073741824, 0, 0, 0, 0, 0, 0, 0, 0, 128849018880, 120259084288, 103079215104, 68719476736, 0, 0, 0, 274877906944, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17592186044416, 0, 35184372088832, 0, 70368744177664, 0, 0, 2269391999729664, 0, 0, 9042383626829824, 0, 0,
    0, 0, 537921536, 0, 0, 0, 1077952512, 0, 0, 0, 0, 537919488, 0, 0, 1077936128, 0, 0, 0, 0, 0, 536870912, 0, 1073741824, 0, 0, 0, 0, 0, 0, 0, 0, 0, 266287972352, 257698037760, 240518168576, 206158430208, 137438953472, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35184372088832, 0, 70368744177664, 0, 0, 0, 0, 4538783999459328, 0, 0, 18084767253659648, 0,
    0, 0, 0, 1075843072, 0, 0, 0, 2155905024, 0, 0, 0, 0, 1075838976, 0, 0, 2155872256, 0, 0, 0, 0, 0, 1073741824, 0, 2147483648, 0, 0, 0, 0, 0, 0, 0, 0, 541165879296, 532575944704, 515396075520, 481036337152, 412316860416, 274877906944, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 70368744177664, 0, 140737488355328, 0, 0, 0, 0, 9077567998918656, 0, 0, 36169534507319296,
    4311810304, 0, 0, 0, 0, 8657571840, 0, 0, 4311810048, 0, 0, 0, 8657567744, 0, 0, 0, 4311744512, 0, 0, 8657043456, 0, 0, 0, 0, 4294967296, 0, 8589934592, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2199023255552, 6597069766656, 15393162788864, 32985348833280, 68169720922112, 138538465099776, 0, 0, 0, 0, 0, 0, 0, 0, 281474976710656, 0, 562949953421312, 0, 0, 0, 0, 0,
    0, 8623620608, 0, 0, 0, 0, 17315143680, 0, 0, 8623620096, 0, 0, 0, 17315135488, 0, 0, 0, 8623489024, 0, 0, 17314086912, 0, 0, 0, 0, 8589934592, 0, 17179869184, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4398046511104, 13194139533312, 30786325577728, 65970697666560, 136339441844224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 562949953421312, 0, 1125899906842624, 0, 0, 0, 0,
    0, 0, 17247241216, 0, 0, 0, 0, 34630287360, 0, 0, 17247240192, 0, 0, 0, 34630270976, 0, 0, 0, 17246978048, 0, 0, 34628173824, 0, 0, 8589934592, 0, 17179869184, 0, 34359738368, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2199023255552, 0, 0, 0, 8796093022208, 26388279066624, 61572651155456, 131941395333120, 0, 0, 0, 0, 0, 0, 0, 0, 562949953421312, 0, 1125899906842624, 0, 2251799813685248, 0, 0, 0,
    0, 0, 0, 34494482432, 0, 0, 0, 0, 0, 0, 0, 34494480384, 0, 0, 0, 69260541952, 17213423616, 0, 0, 34493956096, 0, 0, 69256347648, 0, 0, 17179869184, 0, 34359738368, 0, 68719476736, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6597069766656, 4398046511104, 0, 0, 0, 17592186044416, 52776558133248, 123145302310912, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1125899906842624, 0, 2251799813685248, 0, 4503599627370496, 0, 0,
    0, 0, 0, 0, 68988964864, 0, 0, 0, 34426978304, 0, 0, 0, 68988960768, 0, 0, 0, 0, 34426847232, 0, 0, 68987912192, 0, 0, 138512695296, 0, 0, 34359738368, 0, 68719476736, 0, 137438953472, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15393162788864, 13194139533312, 8796093022208, 0, 0, 0, 35184372088832, 105553116266496, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2251799813685248, 0, 4503599627370496, 0, 9007199254740992, 0,
    68853957120, 0, 0, 0, 0, 137977929728, 0, 0, 0, 68853956608, 0, 0, 0, 137977921536, 0, 0, 0, 0, 68853694464, 0, 0, 137975824384, 0, 0, 0, 0, 0, 68719476736, 0, 137438953472, 0, 274877906944, 0, 0, 0, 0, 0, 0, 0, 0, 32985348833280, 30786325577728, 26388279066624, 17592186044416, 0, 0, 0, 70368744177664, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4503599627370496, 0, 9007199254740992, 0, 18014398509481984,
    0, 137707914240, 0, 0, 0, 0, 275955859456, 0, 0, 0, 137707913216, 0, 0, 0, 275955843072, 0, 0, 0, 0, 137707388928, 0, 0, 275951648768, 0, 0, 0, 0, 0, 137438953472, 0, 274877906944, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68169720922112, 65970697666560, 61572651155456, 52776558133248, 35184372088832, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9007199254740992, 0, 18014398509481984, 0,
    0, 0, 275415828480, 0, 0, 0, 0, 551911718912, 0, 0, 0, 275415826432, 0, 0, 0, 551911686144, 0, 0, 0, 0, 275414777856, 0, 0, 551903297536, 0, 0, 0, 0, 0, 274877906944, 0, 549755813888, 0, 0, 0, 0, 0, 0, 0, 0, 138538465099776, 136339441844224, 131941395333120, 123145302310912, 105553116266496, 70368744177664, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18014398509481984, 0, 36028797018963968,
    1103823438080, 0, 0, 0, 0, 0, 2216338399232, 0, 1103823437824, 0, 0, 0, 0, 2216338391040, 0, 0, 1103823372288, 0, 0, 0, 2216337342464, 0, 0, 0, 1103806595072, 0, 0, 2216203124736, 0, 0, 0, 0, 1099511627776, 0, 2199023255552, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 562949953421312, 1688849860263936, 3940649673949184, 8444249301319680, 17451448556060672, 35465847065542656, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 2207646876160, 0, 0, 0, 0, 0, 4432676798464, 0, 2207646875648, 0, 0, 0, 0, 4432676782080, 0, 0, 2207646744576, 0, 0, 0, 4432674684928, 0, 0, 0, 2207613190144, 0, 0, 4432406249472, 0, 0, 0, 0, 2199023255552, 0, 4398046511104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1125899906842624, 3377699720527872, 7881299347898368, 16888498602639360, 34902897112121344, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 4415293752320, 0, 0, 0, 0, 0, 0, 0, 4415293751296, 0, 0, 0, 0, 8865353564160, 0, 0, 4415293489152, 0, 0, 0, 8865349369856, 0, 0, 0, 4415226380288, 0, 0, 8864812498944, 0, 0, 2199023255552, 0, 4398046511104, 0, 8796093022208, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 562949953421312, 0, 0, 0, 2251799813685248, 6755399441055744, 15762598695796736, 33776997205278720, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 8830587504640, 0, 0, 0, 0, 0, 0, 0, 8830587502592, 0, 0, 0, 0, 0, 0, 0, 8830586978304, 0, 0, 0, 17730698739712, 4406636445696, 0, 0, 8830452760576, 0, 0, 17729624997888, 0, 0, 4398046511104, 0, 8796093022208, 0, 17592186044416, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1688849860263936, 1125899906842624, 0, 0, 0, 4503599627370496, 13510798882111488, 31525197391593472, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 17661175009280, 0, 0, 0, 0, 0, 0, 0, 17661175005184, 0, 0, 0, 8813306445824, 0, 0, 0, 17661173956608, 0, 0, 0, 0, 8813272891392, 0, 0, 17660905521152, 0, 0, 35459249995776, 0, 0, 8796093022208, 0, 17592186044416, 0, 35184372088832, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3940649673949184, 3377699720527872, 2251799813685248, 0, 0, 0, 9007199254740992, 27021597764222976, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 35322350018560, 0, 0, 17626613022720, 0, 0, 0, 0, 35322350010368, 0, 0, 0, 17626612891648, 0, 0, 0, 35322347913216, 0, 0, 0, 0, 17626545782784, 0, 0, 35321811042304, 0, 0, 0, 0, 0, 17592186044416, 0, 35184372088832, 0, 70368744177664, 0, 0, 0, 0, 0, 0, 0, 0, 8444249301319680, 7881299347898368, 6755399441055744, 4503599627370496, 0, 0, 0, 18014398509481984, 0, 0, 0, 0, 0, 0, 0, 0,
    35253226045952, 0, 0, 0, 0, 0, 70644700037120, 0, 0, 35253226045440, 0, 0, 0, 0, 70644700020736, 0, 0, 0, 35253225783296, 0, 0, 0, 70644695826432, 0, 0, 0, 0, 35253091565568, 0, 0, 70643622084608, 0, 0, 0, 0, 0, 35184372088832, 0, 70368744177664, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17451448556060672, 16888498602639360, 15762598695796736, 13510798882111488, 9007199254740992, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 70506452091904, 0, 0, 0, 0, 0, 141289400074240, 0, 0, 70506452090880, 0, 0, 0, 0, 141289400041472, 0, 0, 0, 70506451566592, 0, 0, 0, 141289391652864, 0, 0, 0, 0, 70506183131136, 0, 0, 141287244169216, 0, 0, 0, 0, 0, 70368744177664, 0, 140737488355328, 0, 0, 0, 0, 0, 0, 0, 0, 35465847065542656, 34902897112121344, 33776997205278720, 31525197391593472, 27021597764222976, 18014398509481984, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    282578800148736, 0, 0, 0, 0, 0, 0, 567382630219776, 282578800148480, 0, 0, 0, 0, 0, 567382630203392, 0, 282578800082944, 0, 0, 0, 0, 567382628106240, 0, 0, 282578783305728, 0, 0, 0, 567382359670784, 0, 0, 0, 282574488338432, 0, 0, 567347999932416, 0, 0, 0, 0, 281474976710656, 0, 562949953421312, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 144115188075855872, 432345564227567616, 1008806316530991104, 2161727821137838080, 4467570830351532032, 9079256848778919936,
    0, 565157600297472, 0, 0, 0, 0, 0, 0, 0, 565157600296960, 0, 0, 0, 0, 0, 1134765260406784, 0, 565157600165888, 0, 0, 0, 0, 1134765256212480, 0, 0, 565157566611456, 0, 0, 0, 1134764719341568, 0, 0, 0, 565148976676864, 0, 0, 1134695999864832, 0, 0, 0, 0, 562949953421312, 0, 1125899906842624, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 288230376151711744, 864691128455135232, 2017612633061982208, 4323455642275676160, 8935141660703064064,
    0, 0, 1130315200594944, 0, 0, 0, 0, 0, 0, 0, 1130315200593920, 0, 0, 0, 0, 0, 0, 0, 1130315200331776, 0, 0, 0, 0, 2269530512424960, 0, 0, 1130315133222912, 0, 0, 0, 2269529438683136, 0, 0, 0, 1130297953353728, 0, 0, 2269391999729664, 0, 0, 562949953421312, 0, 1125899906842624, 0, 2251799813685248, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 144115188075855872, 0, 0, 0, 576460752303423488, 1729382256910270464, 4035225266123964416, 8646911284551352320,
    0, 0, 0, 2260630401189888, 0, 0, 0, 0, 0, 0, 0, 2260630401187840, 0, 0, 0, 0, 0, 0, 0, 2260630400663552, 0, 0, 0, 0, 0, 0, 0, 2260630266445824, 0, 0, 0, 4539058877366272, 1128098930098176, 0, 0, 2260595906707456, 0, 0, 4538783999459328, 0, 0, 1125899906842624, 0, 2251799813685248, 0, 4503599627370496, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 432345564227567616, 288230376151711744, 0, 0, 0, 1152921504606846976, 3458764513820540928, 8070450532247928832,
    0, 0, 0, 0, 4521260802379776, 0, 0, 0, 0, 0, 0, 0, 4521260802375680, 0, 0, 0, 0, 0, 0, 0, 4521260801327104, 0, 0, 0, 2256206450130944, 0, 0, 0, 4521260532891648, 0, 0, 0, 0, 2256197860196352, 0, 0, 4521191813414912, 0, 0, 9077567998918656, 0, 0, 2251799813685248, 0, 4503599627370496, 0, 9007199254740992, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1008806316530991104, 864691128455135232, 576460752303423488, 0, 0, 0, 2305843009213693952, 6917529027641081856,
    0, 0, 0, 0, 0, 9042521604759552, 0, 0, 0, 0, 0, 0, 0, 9042521604751360, 0, 0, 4512412933816320, 0, 0, 0, 0, 9042521602654208, 0, 0, 0, 4512412900261888, 0, 0, 0, 9042521065783296, 0, 0, 0, 0, 4512395720392704, 0, 0, 9042383626829824, 0, 0, 0, 0, 0, 4503599627370496, 0, 9007199254740992, 0, 18014398509481984, 0, 0, 0, 0, 0, 0, 0, 0, 2161727821137838080, 2017612633061982208, 1729382256910270464, 1152921504606846976, 0, 0, 0, 4611686018427387904,
    0, 0, 0, 0, 0, 0, 18085043209519104, 0, 9024825867763712, 0, 0, 0, 0, 0, 18085043209502720, 0, 0, 9024825867632640, 0, 0, 0, 0, 18085043205308416, 0, 0, 0, 9024825800523776, 0, 0, 0, 18085042131566592, 0, 0, 0, 0, 9024791440785408, 0, 0, 18084767253659648, 0, 0, 0, 0, 0, 9007199254740992, 0, 18014398509481984, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4467570830351532032, 4323455642275676160, 4035225266123964416, 3458764513820540928, 2305843009213693952, 0, 0, 0,
    18049651735527936, 0, 0, 0, 0, 0, 0, 36170086419038208, 0, 18049651735527424, 0, 0, 0, 0, 0, 36170086419005440, 0, 0, 18049651735265280, 0, 0, 0, 0, 36170086410616832, 0, 0, 0, 18049651601047552, 0, 0, 0, 36170084263133184, 0, 0, 0, 0, 18049582881570816, 0, 0, 36169534507319296, 0, 0, 0, 0, 0, 18014398509481984, 0, 36028797018963968, 0, 0, 0, 0, 0, 0, 0, 0, 9079256848778919936, 8935141660703064064, 8646911284551352320, 8070450532247928832, 6917529027641081856, 4611686018427387904, 0, 0,
];


pub fn print_bitboard(value: u64) {
    if value == 0 {
        println!("0");
        return;
    }

    for y in (0..8).rev() {
        print!("{} |", y + 1);
        for x in 0..8 {
            print!("{} ", 1 & (value >> (x + y * 8)));
        }

        println!();
    }

    println!("   a b c d e f g h");
}