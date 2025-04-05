-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

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
-- TABLE: client
-- ------------------------------

DEFINE TABLE client TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD address ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD created_at ON client TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD email ON client TYPE option<string> DEFAULT NONE PERMISSIONS FULL;
DEFINE FIELD first_name ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD last_name ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD phone ON client TYPE option<string> DEFAULT NONE PERMISSIONS FULL;
DEFINE FIELD updated_at ON client TYPE datetime DEFAULT time::now() PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: client
-- ------------------------------

INSERT [ { address: 'Liège', created_at: d'2025-04-05T00:09:05.838240Z', first_name: 'Tamère', id: client:8pe1chh27xynh0d8ml8y, last_name: 'Lapute', updated_at: d'2025-04-05T00:09:05.838245Z' }, { address: 'Paris', created_at: d'2025-04-05T00:08:48.778675Z', first_name: 'Patrick', id: client:icbxobm75z570rmbdj9z, last_name: 'Fiory', updated_at: d'2025-04-05T00:08:48.778682Z' }, { address: 'Charleroi', created_at: d'2025-04-05T00:08:34.621171Z', first_name: 'Vincent', id: client:pbc0t540l03qmukxt20d, last_name: 'Devilers', updated_at: d'2025-04-05T00:08:34.621176Z' }, { address: 'Charleroi', created_at: d'2025-04-04T23:49:08.343295Z', first_name: 'Hugo', id: client:xql8n6t1ude79zed6i5z, last_name: 'Bohy', updated_at: d'2025-04-04T23:49:08.343299Z' } ];

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

INSERT [ { car_id: car:htpf, created_at: d'2025-04-05T00:14:21.447525Z', id: intervention:fea5pqpxq1d994rqt654, intervention_date: d'2022-12-04T00:00:00Z', intervention_type: { Maintenance: { filter_air: false, filter_cabin: false, filter_gasoil: true, filter_oil: false, spark_plug: false } }, mileage: 300dec, price: 10394dec, remark: [], updated_at: d'2025-04-05T00:14:21.447551Z' }, { car_id: car:vnvk, created_at: d'2025-04-05T00:07:17.791043Z', id: intervention:q9iw18uq401wrzv59xvh, intervention_date: d'2025-03-12T00:00:00Z', intervention_type: { Maintenance: { filter_air: true, filter_cabin: true, filter_gasoil: true, filter_oil: true, spark_plug: false } }, mileage: 110000dec, price: 287dec, remark: [], updated_at: d'2025-04-05T00:07:17.791053Z' }, { car_id: car:okli, created_at: d'2025-04-05T00:16:02.832Z', id: intervention:qjllyagzpwzueofbb1fk, intervention_date: d'2025-03-12T00:00:00Z', intervention_type: 'Repair', mileage: 3000000dec, price: 175839dec, remark: [], updated_at: d'2025-04-05T00:16:02.832Z' }, { car_id: car:ahtb, created_at: d'2025-04-05T00:13:27.116116Z', id: intervention:zrctgddzgdvm2x6shlrb, intervention_date: d'2013-07-12T00:00:00Z', intervention_type: { Maintenance: { filter_air: true, filter_cabin: false, filter_gasoil: true, filter_oil: true, spark_plug: false } }, mileage: 60000dec, price: 32dec, remark: [], updated_at: d'2025-04-05T00:13:27.116125Z' } ];

