-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- ANALYZERS
-- ------------------------------

DEFINE ANALYZER client_analyzer TOKENIZERS CLASS,CAMEL FILTERS NGRAM(1,4),LOWERCASE;

-- ------------------------------
-- TABLE: car
-- ------------------------------

DEFINE TABLE car TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD brand ON car TYPE string PERMISSIONS FULL;
DEFINE FIELD cc ON car TYPE decimal PERMISSIONS FULL;
DEFINE FIELD client_id ON car TYPE record<client> PERMISSIONS FULL;
DEFINE FIELD created_at ON car TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD fuel ON car TYPE 'Gasoline' | 'Diesel' | 'Other' PERMISSIONS FULL;
DEFINE FIELD id ON car TYPE record<car> PERMISSIONS FULL;
DEFINE FIELD model ON car TYPE string PERMISSIONS FULL;
DEFINE FIELD oil_quantity ON car TYPE decimal PERMISSIONS FULL;
DEFINE FIELD oil_type ON car TYPE string PERMISSIONS FULL;
DEFINE FIELD updated_at ON car TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD year ON car TYPE int PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: car
-- ------------------------------

INSERT [ { brand: 'mini', cc: 1.5dec, client_id: client:pbc0t540l03qmukxt20d, created_at: d'2025-04-05T00:12:07.405Z', fuel: 'Gasoline', id: car:ahtb, model: 'cooper', oil_quantity: 6.5dec, oil_type: '5W30', updated_at: d'2025-04-05T00:12:07.405Z', year: 2024 }, { brand: 'Citroën', cc: 0.9dec, client_id: client:8pe1chh27xynh0d8ml8y, created_at: d'2025-04-05T00:10:21.501Z', fuel: 'Diesel', id: car:htpf, model: 'C3', oil_quantity: 3dec, oil_type: '5W30', updated_at: d'2025-04-05T00:10:21.501Z', year: 2021 }, { brand: 'fiat', cc: 0.5dec, client_id: client:icbxobm75z570rmbdj9z, created_at: d'2025-04-05T00:11:09.807Z', fuel: 'Other', id: car:okli, model: 'punto', oil_quantity: 2dec, oil_type: '5W30', updated_at: d'2025-04-05T00:11:09.807Z', year: 2006 }, { brand: 'nissan', cc: 0.9dec, client_id: client:xql8n6t1ude79zed6i5z, created_at: d'2025-04-04T23:54:01.276Z', fuel: 'Gasoline', id: car:vnvk, model: 'micra', oil_quantity: 4.5dec, oil_type: '5W30', updated_at: d'2025-04-04T23:54:01.276Z', year: 2018 } ];

-- ------------------------------
-- TABLE: car_owner_history
-- ------------------------------

DEFINE TABLE car_owner_history TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD car ON car_owner_history TYPE record<car> PERMISSIONS FULL;
DEFINE FIELD client ON car_owner_history TYPE record<client> PERMISSIONS FULL;
DEFINE FIELD created_at ON car_owner_history TYPE datetime DEFAULT time::now() PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: car_owner_history
-- ------------------------------


-- ------------------------------
-- TABLE: client
-- ------------------------------

DEFINE TABLE client TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD address ON client TYPE object PERMISSIONS FULL;
DEFINE FIELD address.country ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD address.number ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD address.postal ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD address.street ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD created_at ON client TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD email ON client TYPE option<string> DEFAULT NONE PERMISSIONS FULL;
DEFINE FIELD first_name ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD full_name ON client TYPE string DEFAULT string::concat(client.first_name, client.last_name) PERMISSIONS FULL;
DEFINE FIELD last_name ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD phone ON client TYPE option<string> DEFAULT NONE PERMISSIONS FULL;
DEFINE FIELD updated_at ON client TYPE datetime DEFAULT time::now() PERMISSIONS FULL;

DEFINE INDEX client_email ON client FIELDS email SEARCH ANALYZER client_analyzer BM25(1.2,0.75) DOC_IDS_ORDER 100 DOC_LENGTHS_ORDER 100 POSTINGS_ORDER 100 TERMS_ORDER 100 DOC_IDS_CACHE 100 DOC_LENGTHS_CACHE 100 POSTINGS_CACHE 100 TERMS_CACHE 100;
DEFINE INDEX client_first_name ON client FIELDS first_name SEARCH ANALYZER client_analyzer BM25(1.2,0.75) DOC_IDS_ORDER 100 DOC_LENGTHS_ORDER 100 POSTINGS_ORDER 100 TERMS_ORDER 100 DOC_IDS_CACHE 100 DOC_LENGTHS_CACHE 100 POSTINGS_CACHE 100 TERMS_CACHE 100;
DEFINE INDEX client_full_name ON client FIELDS full_name SEARCH ANALYZER client_analyzer BM25(1.2,0.75) DOC_IDS_ORDER 100 DOC_LENGTHS_ORDER 100 POSTINGS_ORDER 100 TERMS_ORDER 100 DOC_IDS_CACHE 100 DOC_LENGTHS_CACHE 100 POSTINGS_CACHE 100 TERMS_CACHE 100;
DEFINE INDEX client_last_name ON client FIELDS last_name SEARCH ANALYZER client_analyzer BM25(1.2,0.75) DOC_IDS_ORDER 100 DOC_LENGTHS_ORDER 100 POSTINGS_ORDER 100 TERMS_ORDER 100 DOC_IDS_CACHE 100 DOC_LENGTHS_CACHE 100 POSTINGS_CACHE 100 TERMS_CACHE 100;
DEFINE INDEX client_phone ON client FIELDS phone SEARCH ANALYZER client_analyzer BM25(1.2,0.75) DOC_IDS_ORDER 100 DOC_LENGTHS_ORDER 100 POSTINGS_ORDER 100 TERMS_ORDER 100 DOC_IDS_CACHE 100 DOC_LENGTHS_CACHE 100 POSTINGS_CACHE 100 TERMS_CACHE 100;

DEFINE EVENT full_name ON client WHEN $event = 'CREATE' OR ($event = 'UPDATE' AND ($before.first_name != $after.first_name OR $before.last_name != $after.last_name)) THEN {
LET $new_full_name = string::concat($after.first_name, ' ', $after.last_name);
UPDATE $after.id SET full_name = $new_full_name;
};

-- ------------------------------
-- TABLE DATA: client
-- ------------------------------

INSERT [ { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-17T22:01:55.289Z', first_name: 'Hugo-Antoinette', full_name: 'Hugo-Antoinette je-sais', id: client:2favq3j60bybfv7211b2, last_name: 'je-sais', updated_at: d'2025-04-17T22:01:55.289Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-21T11:50:51.679578043Z', first_name: 'un', full_name: 'un deux', id: client:4v1t7riysqnqnucx01rt, last_name: 'deux', updated_at: d'2025-04-21T11:50:51.679876001Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-21T12:23:29.666732630Z', first_name: 'alkjzhekh', full_name: 'alkjzhekh mama', id: client:5oto5yrchbsp64tj8qqd, last_name: 'mama', updated_at: d'2025-04-21T12:23:29.667010838Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-14T21:49:29.885Z', first_name: 'oki', full_name: 'oki peout', id: client:6wre2175do2lwm1dp6re, last_name: 'peout', updated_at: d'2025-04-14T21:49:29.885Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-05T00:09:05.838Z', first_name: 'Tamèrede', full_name: 'Tamèrede Lapute', id: client:8pe1chh27xynh0d8ml8y, last_name: 'Lapute', updated_at: d'2025-04-05T00:09:05.838Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-21T12:26:23.182420168Z', first_name: 'bbbb', full_name: 'bbbb ccccc', id: client:cbv5co545pzv7teef7qz, last_name: 'ccccc', updated_at: d'2025-04-21T12:26:23.182560126Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-05T00:08:48.778Z', first_name: 'Patricks', full_name: 'Patricks Fiory', id: client:icbxobm75z570rmbdj9z, last_name: 'Fiory', updated_at: d'2025-04-05T00:08:48.778Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-21T11:51:14.883917262Z', first_name: 'toi', full_name: 'toi moi', id: client:inz0y843t5ig9teenl6u, last_name: 'moi', updated_at: d'2025-04-21T11:51:14.884003345Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-05T00:08:34.621Z', first_name: 'Vinz', full_name: 'Vinz Devilers', id: client:pbc0t540l03qmukxt20d, last_name: 'Devilers', updated_at: d'2025-04-05T00:08:34.621Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-21T12:23:55.873327128Z', first_name: 'test final ', full_name: "test final  c'est la bonne ", id: client:sxiaujao6ukvi1e48366, last_name: "c'est la bonne ", updated_at: d'2025-04-21T12:23:55.873491836Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-19T07:32:12.189345919Z', email: 'mdevilers@skynet.be', first_name: 'azkheblakbrkalb', full_name: 'azkheblakbrkalb lkhkahzgelhajkgrhj', id: client:tw1rlqv7cckfee0tpvso, last_name: 'lkhkahzgelhajkgrhj', phone: '213456', updated_at: d'2025-04-19T07:32:12.189568460Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-21T12:24:37.368182717Z', first_name: 'final du final', full_name: 'final du final wallah', id: client:u7q2bmqckg410qrf5ww1, last_name: 'wallah', updated_at: d'2025-04-21T12:24:37.368273175Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-17T22:02:33.896908590Z', first_name: 'aoizehiagroigar', full_name: 'aoizehiagroigar mama', id: client:ueksy4qka2eq2n2p4t5c, last_name: 'mama', updated_at: d'2025-04-17T22:02:33.897287590Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-04T23:49:08.343295Z', first_name: 'Hugo', full_name: 'Hugo Bohy', id: client:xql8n6t1ude79zed6i5z, last_name: 'Bohy', updated_at: d'2025-04-04T23:49:08.343299Z' }, { address: { country: 'Belgium', number: '177', postal: '6061', street: 'Rue de la paix' }, created_at: d'2025-04-21T12:26:11.599457469Z', first_name: 'aaaaa', full_name: 'aaaaa vvvvv', id: client:xyrxj05ck9fhwzfo08x9, last_name: 'vvvvv', updated_at: d'2025-04-21T12:26:11.599641094Z' } ];

-- ------------------------------
-- TABLE: intervention
-- ------------------------------

DEFINE TABLE intervention TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD car_id ON intervention TYPE record<car> PERMISSIONS FULL;
DEFINE FIELD client ON intervention TYPE record<client> PERMISSIONS FULL;
DEFINE FIELD created_at ON intervention TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD intervention_date ON intervention TYPE datetime PERMISSIONS FULL;
DEFINE FIELD intervention_type ON intervention FLEXIBLE TYPE { Maintenance: { filter_air: bool, filter_cabin: bool, filter_gasoil: bool, filter_oil: bool, spark_plug: bool } } | 'Repair' PERMISSIONS FULL;
DEFINE FIELD mileage ON intervention TYPE decimal PERMISSIONS FULL;
DEFINE FIELD price ON intervention TYPE decimal PERMISSIONS FULL;
DEFINE FIELD remark ON intervention TYPE array<string> PERMISSIONS FULL;
DEFINE FIELD remark[*] ON intervention TYPE string PERMISSIONS FULL;
DEFINE FIELD updated_at ON intervention TYPE datetime DEFAULT time::now() PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: intervention
-- ------------------------------

INSERT [ { car_id: car:htpf, client: client:8pe1chh27xynh0d8ml8y, created_at: d'2025-04-05T00:14:21.447Z', id: intervention:fea5pqpxq1d994rqt654, intervention_date: d'2022-12-04T00:00:00Z', intervention_type: { Maintenance: { filter_air: false, filter_cabin: false, filter_gasoil: true, filter_oil: false, spark_plug: false } }, mileage: 300dec, price: 10394dec, remark: [], updated_at: d'2025-04-05T00:14:21.447Z' }, { car_id: car:vnvk, client: client:xql8n6t1ude79zed6i5z, created_at: d'2025-04-05T00:07:17.791Z', id: intervention:q9iw18uq401wrzv59xvh, intervention_date: d'2025-03-12T00:00:00Z', intervention_type: { Maintenance: { filter_air: true, filter_cabin: true, filter_gasoil: true, filter_oil: true, spark_plug: false } }, mileage: 110000dec, price: 287dec, remark: [], updated_at: d'2025-04-05T00:07:17.791Z' }, { car_id: car:okli, client: client:icbxobm75z570rmbdj9z, created_at: d'2025-04-05T00:16:02.832Z', id: intervention:qjllyagzpwzueofbb1fk, intervention_date: d'2025-03-12T00:00:00Z', intervention_type: 'Repair', mileage: 3000000dec, price: 175839dec, remark: [], updated_at: d'2025-04-05T00:16:02.832Z' }, { car_id: car:ahtb, client: client:pbc0t540l03qmukxt20d, created_at: d'2025-04-05T00:13:27.116Z', id: intervention:zrctgddzgdvm2x6shlrb, intervention_date: d'2013-07-12T00:00:00Z', intervention_type: { Maintenance: { filter_air: true, filter_cabin: false, filter_gasoil: true, filter_oil: true, spark_plug: false } }, mileage: 60000dec, price: 32dec, remark: [], updated_at: d'2025-04-05T00:13:27.116Z' } ];

