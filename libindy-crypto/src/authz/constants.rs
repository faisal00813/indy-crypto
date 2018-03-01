pub const R_0_SIZE: usize = 256;
pub const P_SIZE: usize = 256;
pub const POLICY_ADDRESS_SIZE: usize = 256;
pub const SECRET_SIZE: usize = 256;
pub const ACCUM_MODULUS_SIZE: usize = 4096;
pub const GENERATOR_SIZE: usize = 2048;

// P_1, P_2 and P_3 are safe primes satisfying
// P_1 = 2P_0 + 1; P_2 = 2P_1 + 1; P_3 = 2P_2 + 1
pub const P_1: &str = "201534608003891207648831571443070482656058874463389132627396685790245748679899";
pub const P_2: &str = "403069216007782415297663142886140965312117748926778265254793371580491497359799";
pub const P_3: &str = "806138432015564830595326285772281930624235497853556530509586743160982994719599";

pub const G_1: &str = "188595418671626503071098712518267422095874335964641520157433070855758594321392";
pub const H_1: &str = "92402599985214902147414976941886711341909038942387023945335317298079824605630";
pub const G_2: &str = "233119112495040759509149177106855508717462239072487648820500348392299149087831";
pub const H_2: &str = "18319858356219620005264402149827958684192198496003706799409356008492336903275";
pub const G_3: &str = "424351437137216862483190913838749023165013229325879843712895441669675261041532";
pub const H_3: &str = "103793089579771809629014885646834969672838107376656710141244858150369985189175";

pub const ACCUM1_MODULUS: &str = "3204629348400311894981147889064213475911502950610822910497794640539580108252311103815602649702091072442089016826192705629222534383850640976103541074965825911439481511841669818218556523699545667979238433531950872370285535430396347665030321087803912182039011247460768098263564546879241877558066260767468387251243972651463237193324761892342299558684399419909028079667195672054655387173203038107042778744649155842364672936664532754000715885900524415403622527226575507346822066350920781137340258848229685505555558665781467456342659911415599377043489979596237979941386501264463874766166877146485965674900595259972161874714381016252770855749700706860768647580178487242906896883599546187195495868346449603306725234279605044720570340541521141682943069084999263530360914420274990375408153529679146137934097158741957518183323298580558730909779698051995609396227449697670716584357361212378371486020465912177797480071144296559699016912326564059836301570632836543118917475972178423287627710340103368809606657431184660768586693677060010145686280797896075497757615632909442703186036300871183851569319692562563918216157762976445282183184528709786306196550055260857987968870798605413506633826598403862811608523439523945519737279157633473140586034211201";
pub const ACCUM2_MODULUS: &str = "3133032080924566959881637452191055160609655524293438819606385720674991596361894869941623022225068918605348030821215520086114403808621280850053350684795125712710087407012772894036421425862560233314314648523623147277999548688559474388879061877185548725118089223823093604853029430403643133732826444283877030142730436440806724215015138813375919445754261729440238895393116059687080047420951044300698828127667739793319375589516868216652624004044627898341937289739315504685938380570351880496638545491909140937856863007811260788811375608394518915419894526073205731567863017762549443787949066965061546426190072082119595112722987549714336192426168846730850256555201637411054474942106140136887446606712629901233511758478704789426507699648599228568443716000256178232838173172583529219810703502190858655879340238882906600390117959887174936452538004363459231760721550955196985907009422925598088870254721647538456683476170050788346184902057773346576532359413370880902716172056325843377330639613774508721179639675066091253808533650050983242491653107140358137874776494507066715820367214766161602433720424953426293579075167196506903155447532819416895145183616697787460722795095532351341246466256478197255754918183172139858779830733597837907514550395373";

pub const ACCUM1_MODULUS_BY_4: &str = "801157337100077973745286972266053368977875737652705727624448660134895027063077775953900662425522768110522254206548176407305633595962660244025885268741456477859870377960417454554639130924886416994809608382987718092571383857599086916257580271950978045509752811865192024565891136719810469389516565191867096812810993162865809298331190473085574889671099854977257019916798918013663846793300759526760694686162288960591168234166133188500178971475131103850905631806643876836705516587730195284335064712057421376388889666445366864085664977853899844260872494899059494985346625316115968691541719286621491418725148814993040468678595254063192713937425176715192161895044621810726724220899886546798873967086612400826681308569901261180142585135380285420735767271249815882590228605068747593852038382419786534483524289685489379545830824645139682727444924512998902349056862424417679146089340303094592871505116478044449370017786074139924754228081641014959075392658209135779729368993044605821906927585025842202401664357796165192146673419265002536421570199474018874439403908227360675796509075217795962892329923140640979554039440744111320545796132177446576549137513815214496992217699651353376658456649600965702902130859880986379934319789408368285146508552800";
pub const ACCUM2_MODULUS_BY_4: &str = "783258020231141739970409363047763790152413881073359704901596430168747899090473717485405755556267229651337007705303880021528600952155320212513337671198781428177521851753193223509105356465640058328578662130905786819499887172139868597219765469296387181279522305955773401213257357600910783433206611070969257535682609110201681053753784703343979861438565432360059723848279014921770011855237761075174707031916934948329843897379217054163156001011156974585484322434828876171484595142587970124159636372977285234464215751952815197202843902098629728854973631518301432891965754440637360946987266741265386606547518020529898778180746887428584048106542211682712564138800409352763618735526535034221861651678157475308377939619676197356626924912149807142110929000064044558209543293145882304952675875547714663969835059720726650097529489971793734113134501090864807940180387738799246476752355731399522217563680411884614170869042512697086546225514443336644133089853342720225679043014081460844332659903443627180294909918766522813452133412512745810622913276785089534468694123626766678955091803691540400608430106238356573394768791799126725788861883204854223786295904174446865180698773883087835311616564119549313938729545793034964694957683399459476878637598843";