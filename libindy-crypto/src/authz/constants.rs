pub const ACCUM_A_SIZE: usize = 642;
pub const ACCUM_B_SIZE: usize = 1026;
pub const R_0_SIZE: usize = 1024;
pub const P_0_SIZE: usize = 1024;
pub const P_3_SIZE: usize = 1536;
pub const POLICY_ADDRESS_SIZE: usize = 256;
pub const SECRET_SIZE: usize = 256;
pub const ACCUM_MODULUS_SIZE: usize = 4096;
pub const GENERATOR_SIZE: usize = 2048;

pub const ACCUM1_A_VALUE: &str = "17339984463504597255212769849309949355608390640556012007409825283529588256065267054457188869650861222989142721211669584697058348210099042025711710106820384240272068204079280357154765659038369653";
pub const ACCUM1_B_VALUE: &str = "649721354393551462184542409373543228031458839922593984336262046815810696484615036750817323204933663791018532198449898483036273096362548337463492388429168170397334994106422439493884639024192180599923491510141842023182662687672791858006741143123987044702205714656950718765922714641784994722453213673139404638693";
pub const ACCUM2_A_VALUE: &str = "17652874584182533265304557441213552184061425843317868266662011775230141036955818759904126202735809820711786630780235566106610936019903087318606725100825566920193523539290053733296603624416002317";
pub const ACCUM2_B_VALUE: &str = "579110690705755135913160674049475889163906079722070733972942531889008266199629559926078125668201096016522839756463621700641925093769720957676083768193823263880677876001904069321210767147457121958973223374198668940282006997842179845223697849120237815589652099502415283106548314827953930474159931259528883031007";

// P_1, P_2 and P_3 are safe primes satisfying
// P_1 = 2P_0 + 1; P_2 = 2P_1 + 1; P_3 = 2P_2 + 1
pub const P_0: &str = "342746746630198769587188362941633361135560124689646425521680977506225322276387227741067034544479094411714004599311120760485035571655778439484768711370405653282832105442822212127931391170907193584210573852292058253615408787319187355584026249235847398250452645512426120385580225504856590490940029951599115616979";
pub const P_1: &str = "685493493260397539174376725883266722271120249379292851043361955012450644552774455482134069088958188823428009198622241520970071143311556878969537422740811306565664210885644424255862782341814387168421147704584116507230817574638374711168052498471694796500905291024852240771160451009713180981880059903198231233959";
pub const P_2: &str = "1370986986520795078348753451766533444542240498758585702086723910024901289105548910964268138177916377646856018397244483041940142286623113757939074845481622613131328421771288848511725564683628774336842295409168233014461635149276749422336104996943389593001810582049704481542320902019426361963760119806396462467919";
pub const P_3: &str = "4806876214089177439121678559764069543282270755154137981051366776821330958611719328037311759924923156830623290278296826263863902327008664143707117531049168010908663795201825132050017581985031718536424081509084930569115857201636971728388275433540277562846153879803474020036767852693656753257597801227199822164846876100177774044259379232968071371318658371230787073384750022830829873718254139779006439569882904712552834431199870749249168775012460891012776977366721903";

pub const G_1_1: &str = "400866701921040617156215048717812802171985796055158830616270693805023258821659580054397180603907879004527907118137149748571984685294952305796326513632382882576396186876299632333980659176626303375932150091225393283907484851724158794238201145279388945938468405289217062650266816341582923202034307968434113073327";
pub const G_1_2: &str = "538993185306916276154466017509369228253121126392816627255050915402686002830956863078809986059337879420676268628934899847245467867994297044357514166593217556153252236970100198829583389637273856711362048051824558869438986634348842403123505414092032214542125509035739064481581500809673217798444102490660008227931";
pub const G_1_3: &str = "130452799838656133538357582389660237286750343592794293927733871250947332649177326657392679881148277178897706584890618535318221305474954040315362608236648891572496606713518495314069907076715709417095666766105313313183182170629410786668593105310278849262951009657722005927917113917641586350739462739673622475389";
pub const G_2_1: &str = "69152081823559996481545546999387574252373149478155322100342055441179765088363327741515830995337263561018164401721580953025827651091492575646616474872147531856415616826450923311319872712101497630028414731898085953503485780976953636353247282569364584593975256906843721848226915099059836889612558629884944915008";
pub const G_2_2: &str = "470029649677814242192503197041333182859338474866139735540340237860386985383858769343313634747854052599700657768556593452739840319776906303115462644839907321418286503577728324069632127691102884002681705284561358362477323738279524648804002953489647242608691661030303080949574740614697195329566781922909179377461";
pub const G_2_3: &str = "785393305865513927095272515052529604085540067868964412509556926944118603565786259921281069071449631513788278983039162045376878476554441232101772080607120360522157821165939894077084903205016061602306769081738636293576622819317411306539529915206798601893114841394018765445299690745036010297352223577612968436098";
pub const G_3_1: &str = "3318329373501734845646928325220668696437602201726038156399921742536351228214547018531151044373827188871089409665932489889162298282245229148799472317181289657229126052441864789797810602091219346610409481883083033639737748939225544299895127606170168839505876815930359584132112647021626308916893763395449931467792374755784088661145191903863999252042869965342258001780945784196848368375563814443455234469841392646709875817250454956734778601536416725798457755544488698";
pub const G_3_2: &str = "3515716781789571512678788729247741865988079019974058902021362845084824066063226758810128374497869482527575097999477765218474718786608137019405837473657847850107038854989791363243592270484054786871887291676152189885742636075823810894147152384071176192886133657886064743837762022148128467935800981298850438099246364017075773542027937284421598402994854728898189942228336723700150371060800275508115447894622902700910058578272861837137793741465021524115295620361352483";
pub const G_N: &str = "1915182769368569486963976668657418846122016471585262452138689064979880063217142171230525667406114297912997697884669760718181148903891406258652154564474937314809392827351229074850339882653085906709452763512017926382658040877860425398565313118206001571983057266861433787870120516963262872556160137632986790799637800187327525373894833699998137974425675888510468704041321563743266953919273330110447213516904474856325884904918747921700098338253140217925656078220294833113261362981479227102302139442862538529417112227112104830922667359007109709204936925922862592053626140248383988977967399826184116697852122995719423774656923017777301713520425580745782097460248899329835740441429284020663186635354571333690692107185946297929772630086928323118606594588790915139339431940489511850279806818826143040378319619062305629273497273725721368157955512179352674013822087432127910233025333571203452180390613181846035364105352384024395662959142236311712295552933323767329374261596612060957502167831080774021620823718240397211269873903737841346862215246685242302847894894907938934985950908016181737929997451061016202494801606490097096889822976223609532834237496884730642312786447889763148272139117981855545441295507862786712405210727374768044118436617961";
pub const H_N: &str = "2432415093370501402366531148146293382257446164716390439485519047786992913652146123416109091109768637637257626270349212189687393711935973378445281786914909778914226791230640333916695773986760787104511485252440750125725882943122907504244652500697735401291739063996316239479350704430544924838821813156560501197766524726231540233448879625242703997293328594649574577384137445571614757069910154835627881569928278240460983755029275109738746728298721000524114419959993534943589495490803928060325010578493729888210836563868248414072773767919869890334609017305603504519147370053076846654943575111487836610407726347216474857348128102641894539681790453043726012893523499163634812656213855129640176774211740300896957003619474486571896324611647406829526659716983561642230951371425186377662146467168573744340562384626718798900395877050047022464515227622983420182032068926538841167739975061184834306434592079719601363568510354598846051804883219548209129395039947636809320215998304335596885530817589118539983699280447540805728250451261698066293975106481647115238168440671093557006746860461978849314657682123582972586742738503183316661166278491564825408882129607384587665073868007468679493225077890393276626625018528892902950321835209855423176249644609";

pub const ACCUM1_MODULUS: &str = "3204629348400311894981147889064213475911502950610822910497794640539580108252311103815602649702091072442089016826192705629222534383850640976103541074965825911439481511841669818218556523699545667979238433531950872370285535430396347665030321087803912182039011247460768098263564546879241877558066260767468387251243972651463237193324761892342299558684399419909028079667195672054655387173203038107042778744649155842364672936664532754000715885900524415403622527226575507346822066350920781137340258848229685505555558665781467456342659911415599377043489979596237979941386501264463874766166877146485965674900595259972161874714381016252770855749700706860768647580178487242906896883599546187195495868346449603306725234279605044720570340541521141682943069084999263530360914420274990375408153529679146137934097158741957518183323298580558730909779698051995609396227449697670716584357361212378371486020465912177797480071144296559699016912326564059836301570632836543118917475972178423287627710340103368809606657431184660768586693677060010145686280797896075497757615632909442703186036300871183851569319692562563918216157762976445282183184528709786306196550055260857987968870798605413506633826598403862811608523439523945519737279157633473140586034211201";
pub const ACCUM2_MODULUS: &str = "3133032080924566959881637452191055160609655524293438819606385720674991596361894869941623022225068918605348030821215520086114403808621280850053350684795125712710087407012772894036421425862560233314314648523623147277999548688559474388879061877185548725118089223823093604853029430403643133732826444283877030142730436440806724215015138813375919445754261729440238895393116059687080047420951044300698828127667739793319375589516868216652624004044627898341937289739315504685938380570351880496638545491909140937856863007811260788811375608394518915419894526073205731567863017762549443787949066965061546426190072082119595112722987549714336192426168846730850256555201637411054474942106140136887446606712629901233511758478704789426507699648599228568443716000256178232838173172583529219810703502190858655879340238882906600390117959887174936452538004363459231760721550955196985907009422925598088870254721647538456683476170050788346184902057773346576532359413370880902716172056325843377330639613774508721179639675066091253808533650050983242491653107140358137874776494507066715820367214766161602433720424953426293579075167196506903155447532819416895145183616697787460722795095532351341246466256478197255754918183172139858779830733597837907514550395373";

pub const ACCUM1_MODULUS_BY_4: &str = "801157337100077973745286972266053368977875737652705727624448660134895027063077775953900662425522768110522254206548176407305633595962660244025885268741456477859870377960417454554639130924886416994809608382987718092571383857599086916257580271950978045509752811865192024565891136719810469389516565191867096812810993162865809298331190473085574889671099854977257019916798918013663846793300759526760694686162288960591168234166133188500178971475131103850905631806643876836705516587730195284335064712057421376388889666445366864085664977853899844260872494899059494985346625316115968691541719286621491418725148814993040468678595254063192713937425176715192161895044621810726724220899886546798873967086612400826681308569901261180142585135380285420735767271249815882590228605068747593852038382419786534483524289685489379545830824645139682727444924512998902349056862424417679146089340303094592871505116478044449370017786074139924754228081641014959075392658209135779729368993044605821906927585025842202401664357796165192146673419265002536421570199474018874439403908227360675796509075217795962892329923140640979554039440744111320545796132177446576549137513815214496992217699651353376658456649600965702902130859880986379934319789408368285146508552800";
pub const ACCUM2_MODULUS_BY_4: &str = "783258020231141739970409363047763790152413881073359704901596430168747899090473717485405755556267229651337007705303880021528600952155320212513337671198781428177521851753193223509105356465640058328578662130905786819499887172139868597219765469296387181279522305955773401213257357600910783433206611070969257535682609110201681053753784703343979861438565432360059723848279014921770011855237761075174707031916934948329843897379217054163156001011156974585484322434828876171484595142587970124159636372977285234464215751952815197202843902098629728854973631518301432891965754440637360946987266741265386606547518020529898778180746887428584048106542211682712564138800409352763618735526535034221861651678157475308377939619676197356626924912149807142110929000064044558209543293145882304952675875547714663969835059720726650097529489971793734113134501090864807940180387738799246476752355731399522217563680411884614170869042512697086546225514443336644133089853342720225679043014081460844332659903443627180294909918766522813452133412512745810622913276785089534468694123626766678955091803691540400608430106238356573394768791799126725788861883204854223786295904174446865180698773883087835311616564119549313938729545793034964694957683399459476878637598843";

//2^(2*l+2) where l = 128
pub const SECURITY_LEVEL: &str = "463168356949264781694283940034751631413079938662562256157830336031652518559744";
pub const B_HAT: &str = "237563984113248842628292654801650025621660586001817587511600660901320366832633450585689157193126754926641316993138962937295571540999183012025890940920659683340672145588288586564536471370950760792768740189843754451080055067974989234216395468289952826154195031376238530683487839993737572321377989341468923670918598694399651428114620759434288744132273558611880630980483090838483920755431560";