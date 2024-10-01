-- Create an enum for countries
CREATE TYPE country AS ENUM ('Peru', 'Mexico');

-- Create a function to set the created_at timestamp
CREATE OR REPLACE FUNCTION set_created_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.created_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Location Table
CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    country country NOT NULL,
    state VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create trigger for locations
CREATE TRIGGER set_locations_created_at
BEFORE INSERT ON locations
FOR EACH ROW
EXECUTE FUNCTION set_created_at();

-- Driver Table
CREATE TABLE drivers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    license_plate VARCHAR(20) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create trigger for drivers
CREATE TRIGGER set_drivers_created_at
BEFORE INSERT ON drivers
FOR EACH ROW
EXECUTE FUNCTION set_created_at();

-- Driver Images Table
CREATE TABLE driver_images (
    id SERIAL PRIMARY KEY,
    driver_id INTEGER NOT NULL,
    image_url VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (driver_id) REFERENCES drivers(id)
);

-- Create trigger for driver_images
CREATE TRIGGER set_driver_images_created_at
BEFORE INSERT ON driver_images
FOR EACH ROW
EXECUTE FUNCTION set_created_at();

-- Complaint Table
CREATE TABLE complaints (
    id SERIAL PRIMARY KEY,
    driver_id INTEGER NOT NULL,
    location_id INTEGER NOT NULL,
    taxi_application VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (driver_id) REFERENCES drivers(id),
    FOREIGN KEY (location_id) REFERENCES locations(id)
);

-- Create trigger for complaints
CREATE TRIGGER set_complaints_created_at
BEFORE INSERT ON complaints
FOR EACH ROW
EXECUTE FUNCTION set_created_at();

-- Complaint Images Table
CREATE TABLE complaint_images (
    id SERIAL PRIMARY KEY,
    complaint_id INTEGER NOT NULL,
    image_url VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (complaint_id) REFERENCES complaints(id)
);

-- Create trigger for complaint_images
CREATE TRIGGER set_complaint_images_created_at
BEFORE INSERT ON complaint_images
FOR EACH ROW
EXECUTE FUNCTION set_created_at();

-- Create indexes
CREATE INDEX idx_complaints_driver_id ON complaints(driver_id);
CREATE INDEX idx_complaints_location_id ON complaints(location_id);
CREATE INDEX idx_driver_images_driver_id ON driver_images(driver_id);
CREATE INDEX idx_complaint_images_complaint_id ON complaint_images(complaint_id);

-- Insert states for Peru
INSERT INTO locations (country, state) VALUES
('Peru', 'Amazonas'),
('Peru', 'Áncash'),
('Peru', 'Apurímac'),
('Peru', 'Arequipa'),
('Peru', 'Ayacucho'),
('Peru', 'Cajamarca'),
('Peru', 'Callao'),
('Peru', 'Cusco'),
('Peru', 'Huancavelica'),
('Peru', 'Huánuco'),
('Peru', 'Ica'),
('Peru', 'Junín'),
('Peru', 'La Libertad'),
('Peru', 'Lambayeque'),
('Peru', 'Lima'),
('Peru', 'Loreto'),
('Peru', 'Madre de Dios'),
('Peru', 'Moquegua'),
('Peru', 'Pasco'),
('Peru', 'Piura'),
('Peru', 'Puno'),
('Peru', 'San Martín'),
('Peru', 'Tacna'),
('Peru', 'Tumbes'),
('Peru', 'Ucayali');

-- Insert states for Mexico
INSERT INTO locations (country, state) VALUES
('Mexico', 'Aguascalientes'),
('Mexico', 'Baja California'),
('Mexico', 'Baja California Sur'),
('Mexico', 'Campeche'),
('Mexico', 'Chiapas'),
('Mexico', 'Chihuahua'),
('Mexico', 'Coahuila'),
('Mexico', 'Colima'),
('Mexico', 'Durango'),
('Mexico', 'Guanajuato'),
('Mexico', 'Guerrero'),
('Mexico', 'Hidalgo'),
('Mexico', 'Jalisco'),
('Mexico', 'México'),
('Mexico', 'Mexico City'),
('Mexico', 'Michoacán'),
('Mexico', 'Morelos'),
('Mexico', 'Nayarit'),
('Mexico', 'Nuevo León'),
('Mexico', 'Oaxaca'),
('Mexico', 'Puebla'),
('Mexico', 'Querétaro'),
('Mexico', 'Quintana Roo'),
('Mexico', 'San Luis Potosí'),
('Mexico', 'Sinaloa'),
('Mexico', 'Sonora'),
('Mexico', 'Tabasco'),
('Mexico', 'Tamaulipas'),
('Mexico', 'Tlaxcala'),
('Mexico', 'Veracruz'),
('Mexico', 'Yucatán'),
('Mexico', 'Zacatecas');
