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

BEGIN;
INSERT { brand: 'mini', cc: 1.5dec, client_id: client:pbc0t540l03qmukxt20d, created_at: d'2025-04-05T00:12:07.405Z', fuel: 'Gasoline', id: car:ahtb, model: 'cooper', oil_quantity: 6.5dec, oil_type: '5W30', updated_at: d'2025-04-05T00:12:07.405Z', year: 2024 } VERSION d'2025-04-06T13:29:13.665907012Z';
INSERT { brand: 'Citroën', cc: 0.9dec, client_id: client:8pe1chh27xynh0d8ml8y, created_at: d'2025-04-05T00:10:21.501Z', fuel: 'Diesel', id: car:htpf, model: 'C3', oil_quantity: 3dec, oil_type: '5W30', updated_at: d'2025-04-05T00:10:21.501Z', year: 2021 } VERSION d'2025-04-06T13:29:13.665907012Z';
INSERT { brand: 'fiat', cc: 0.5dec, client_id: client:icbxobm75z570rmbdj9z, created_at: d'2025-04-05T00:11:09.807Z', fuel: 'Other', id: car:okli, model: 'punto', oil_quantity: 2dec, oil_type: '5W30', updated_at: d'2025-04-05T00:11:09.807Z', year: 2006 } VERSION d'2025-04-06T13:29:13.665907012Z';
INSERT { brand: 'nissan', cc: 0.9dec, client_id: client:xql8n6t1ude79zed6i5z, created_at: d'2025-04-04T23:54:01.276Z', fuel: 'Gasoline', id: car:vnvk, model: 'micra', oil_quantity: 4.5dec, oil_type: '5W30', updated_at: d'2025-04-04T23:54:01.276Z', year: 2018 } VERSION d'2025-04-06T13:29:13.665907012Z';
COMMIT;

-- ------------------------------
-- TABLE: client
-- ------------------------------

DEFINE TABLE client TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD address ON client TYPE string PERMISSIONS FULL;
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

DEFINE EVENT full_name ON client WHEN $before.first_name != $after.first_name OR $before.last_name != $after.last_name THEN { UPDATE $parent.id SET full_name = string::concat(first_name, ' ', last_name); };

-- ------------------------------
-- TABLE DATA: client
-- ------------------------------

BEGIN;
INSERT { address: 'Liège', created_at: d'2025-04-14T21:49:29.885Z', first_name: 'oki', full_name: 'oki peout', id: client:6wre2175do2lwm1dp6re, last_name: 'peout', updated_at: d'2025-04-14T21:49:29.885Z' } VERSION d'2025-04-14T21:49:43.139604212Z';
INSERT { address: 'Liège', created_at: d'2025-04-05T00:09:05.838Z', first_name: 'Tamèrede', full_name: 'Tamèrede Lapute', id: client:8pe1chh27xynh0d8ml8y, last_name: 'Lapute', updated_at: d'2025-04-05T00:09:05.838Z' } VERSION d'2025-04-14T21:37:07.595304377Z';
INSERT { address: 'Paris', created_at: d'2025-04-05T00:08:48.778Z', first_name: 'Patricks', full_name: 'Patricks Fiory', id: client:icbxobm75z570rmbdj9z, last_name: 'Fiory', updated_at: d'2025-04-05T00:08:48.778Z' } VERSION d'2025-04-14T21:37:42.493984879Z';
INSERT { address: 'Charleroi', created_at: d'2025-04-05T00:08:34.621Z', first_name: 'Vinz', full_name: 'Vinz Devilers', id: client:pbc0t540l03qmukxt20d, last_name: 'Devilers', updated_at: d'2025-04-05T00:08:34.621Z' } VERSION d'2025-04-14T21:45:38.041654752Z';
INSERT { address: 'Charleroi', created_at: d'2025-04-04T23:49:08.343295Z', first_name: 'Hugo', full_name: 'Hugo Bohy', id: client:xql8n6t1ude79zed6i5z, last_name: 'Bohy', updated_at: d'2025-04-04T23:49:08.343299Z' } VERSION d'2025-04-14T21:37:07.595304377Z';
COMMIT;

-- ------------------------------
-- TABLE: intervention
-- ------------------------------

DEFINE TABLE intervention TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD car_id ON intervention TYPE record<car> PERMISSIONS FULL;
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

BEGIN;
INSERT { car_id: car:htpf, created_at: d'2025-04-05T00:14:21.447525Z', id: intervention:fea5pqpxq1d994rqt654, intervention_date: d'2022-12-04T00:00:00Z', intervention_type: { Maintenance: { filter_air: false, filter_cabin: false, filter_gasoil: true, filter_oil: false, spark_plug: false } }, mileage: 300dec, price: 10394dec, remark: [], updated_at: d'2025-04-05T00:14:21.447551Z' } VERSION d'2025-04-06T13:29:13.672000596Z';
INSERT { car_id: car:vnvk, created_at: d'2025-04-05T00:07:17.791043Z', id: intervention:q9iw18uq401wrzv59xvh, intervention_date: d'2025-03-12T00:00:00Z', intervention_type: { Maintenance: { filter_air: true, filter_cabin: true, filter_gasoil: true, filter_oil: true, spark_plug: false } }, mileage: 110000dec, price: 287dec, remark: [], updated_at: d'2025-04-05T00:07:17.791053Z' } VERSION d'2025-04-06T13:29:13.672000596Z';
INSERT { car_id: car:okli, created_at: d'2025-04-05T00:16:02.832Z', id: intervention:qjllyagzpwzueofbb1fk, intervention_date: d'2025-03-12T00:00:00Z', intervention_type: 'Repair', mileage: 3000000dec, price: 175839dec, remark: [], updated_at: d'2025-04-05T00:16:02.832Z' } VERSION d'2025-04-06T13:29:13.672000596Z';
INSERT { car_id: car:ahtb, created_at: d'2025-04-05T00:13:27.116116Z', id: intervention:zrctgddzgdvm2x6shlrb, intervention_date: d'2013-07-12T00:00:00Z', intervention_type: { Maintenance: { filter_air: true, filter_cabin: false, filter_gasoil: true, filter_oil: true, spark_plug: false } }, mileage: 60000dec, price: 32dec, remark: [], updated_at: d'2025-04-05T00:13:27.116125Z' } VERSION d'2025-04-06T13:29:13.672000596Z';
COMMIT;

