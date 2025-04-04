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
DEFINE FIELD fuel ON car TYPE 'gasoline' | 'diesel' | 'other' PERMISSIONS FULL;
DEFINE FIELD id ON car TYPE record<car> PERMISSIONS FULL;
DEFINE FIELD model ON car TYPE string PERMISSIONS FULL;
DEFINE FIELD oil_quantity ON car TYPE decimal PERMISSIONS FULL;
DEFINE FIELD oil_type ON car TYPE string PERMISSIONS FULL;
DEFINE FIELD year ON car TYPE int PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: car
-- ------------------------------


-- ------------------------------
-- TABLE: client
-- ------------------------------

DEFINE TABLE client TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD adress ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD email ON client TYPE option<string> DEFAULT NONE PERMISSIONS FULL;
DEFINE FIELD first_name ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD last_name ON client TYPE string PERMISSIONS FULL;
DEFINE FIELD phone ON client TYPE option<string> DEFAULT NONE PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: client
-- ------------------------------


-- ------------------------------
-- TABLE: intervention
-- ------------------------------

DEFINE TABLE intervention TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD car_id ON intervention TYPE record<car> PERMISSIONS FULL;
DEFINE FIELD intervention_date ON intervention TYPE datetime PERMISSIONS FULL;
DEFINE FIELD intervention_type ON intervention TYPE { Maintenance: { filter_air: bool, filter_cabin: bool, filter_gasoil: bool, filter_oil: bool, spark_plug: bool } } | { Repair: { fuck: bool } } PERMISSIONS FULL;
DEFINE FIELD mileage ON intervention TYPE decimal PERMISSIONS FULL;
DEFINE FIELD price ON intervention TYPE decimal PERMISSIONS FULL;
DEFINE FIELD remark ON intervention TYPE array<string> PERMISSIONS FULL;
DEFINE FIELD remark[*] ON intervention TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: intervention
-- ------------------------------


