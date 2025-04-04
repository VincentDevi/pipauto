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
DEFINE FIELD id ON car TYPE record<car> PERMISSIONS FULL;
DEFINE FIELD model ON car TYPE string PERMISSIONS FULL;
DEFINE FIELD oil_quantity ON car TYPE decimal PERMISSIONS FULL;
DEFINE FIELD oil_type ON car TYPE string PERMISSIONS FULL;
DEFINE FIELD year ON car TYPE int PERMISSIONS FULL;



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
-- TABLE: maintenance
-- ------------------------------

DEFINE TABLE maintenance TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD car_id ON maintenance TYPE record<car> PERMISSIONS FULL;
DEFINE FIELD filter_air ON maintenance TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD filter_gasoil ON maintenance TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD filter_oil ON maintenance TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD maintenance_date ON maintenance TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD mileage ON maintenance TYPE int PERMISSIONS FULL;
DEFINE FIELD oil ON maintenance TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD price ON maintenance TYPE decimal PERMISSIONS FULL;
DEFINE FIELD remarks ON maintenance TYPE array<string> PERMISSIONS FULL;
DEFINE FIELD remarks[*] ON maintenance TYPE string PERMISSIONS FULL;
DEFINE FIELD spark_plug ON maintenance TYPE bool DEFAULT false PERMISSIONS FULL;



-- ------------------------------
-- TABLE: repair
-- ------------------------------

DEFINE TABLE repair TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD car_id ON repair TYPE record<car> PERMISSIONS FULL;
DEFINE FIELD mileage ON repair TYPE decimal PERMISSIONS FULL;
DEFINE FIELD price ON repair TYPE decimal PERMISSIONS FULL;
DEFINE FIELD remarks ON repair TYPE array<string> PERMISSIONS FULL;
DEFINE FIELD remarks[*] ON repair TYPE string PERMISSIONS FULL;
DEFINE FIELD repair_date ON repair TYPE datetime PERMISSIONS FULL;



