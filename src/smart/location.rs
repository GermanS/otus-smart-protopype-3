use core::fmt;
use std::{error::Error, sync::Arc};

use crate::smart::Pluggable;
use crate::smart::Reportable;

/// Структура `SmartHouse` представляет собой умный дом, содержащий
/// информацию о его названии и комнатах.
///
/// Умный дом может включать в себя несколько умных комнат, каждая из
/// которых может иметь свои особенности и функциональность. Структура
/// предназначена для управления умным домом в целом, предоставляя
/// пользователю возможность взаимодействовать с различными комнатами
/// и их устройствами.
///
/// # Поля
///
/// - `name`: `String`
///
///   Название умного дома. Это поле используется для идентификации
///   дома и может отображаться в пользовательском интерфейсе или при
///   выводе информации о доме.
///
/// - `rooms`: `Vec<SmartRoom>`
///
///   Вектор, содержащий умные комнаты. Каждая комната представлена
///   экземпляром структуры `SmartRoom`, что позволяет управлять
///   ее устройствами и функциональностью.
///
/// # Пример
///
/// ```rust
/// use lesson_4::smart::location::{SmartHouse, SmartRoom};
/// // Предполагается, что структура SmartRoom уже определена.
/// let kitchen = SmartRoom::new(String::from("Kitchen"));
/// let living_room = SmartRoom::new(String::from("Living Room"));
///
/// let mut smart_house = SmartHouse::new(String::from("My Smart Home"));
/// smart_house.add( living_room );
/// ```
#[derive(Clone)]
pub struct SmartHouse {
    name: String,
    rooms: Vec<SmartRoom>,
}

impl SmartHouse {
    /// Создает новый экземпляр `SmartHouse`.
    ///
    /// Этот метод является конструктором для структуры `SmartHouse` и
    /// используется для инициализации нового умного дома с заданным именем.
    /// Новый дом будет содержать пустой вектор комнат, который может быть
    /// заполняем по мере необходимости.
    ///
    /// # Параметры
    ///
    /// - `name`: `String` — Название умного дома. Это поле будет
    ///   использоваться для идентификации дома.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает новый экземпляр `SmartHouse`, инициализированный
    /// заданным именем и пустым вектором комнат.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use lesson_4::smart::location::SmartHouse;
    /// let smart_house = SmartHouse::new(String::from("My Smart Home"));
    /// println!("Smart House Name: {}", smart_house.name());
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: Vec::default(),
        }
    }

    /// Возвращает имя умного дома.
    ///
    /// Этот метод предоставляет доступ к имени экземпляра `SmartHouse`.
    /// Он возвращает ссылку на строку, содержащую название дома.
    /// Метод используется, чтобы получить информацию о названии дома,
    /// не меняя само значение.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает `&str` — ссылку на строковый слайс, представляющую имя
    /// умного дома. Это значение может быть использовано для
    /// вывода информации в интерфейсе или для логирования.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use lesson_4::smart::location::SmartHouse;
    /// let smart_house = SmartHouse::new(String::from("My Smart Home"));
    /// println!("Smart House Name: {}", smart_house.name());
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Добавляет новую умную комнату в дом.
    ///
    /// Этот метод позволяет добавлять экземпляр `SmartRoom` в вектор
    /// комнат умного дома. Перед добавлением метода проверяет, существует
    /// ли уже комната с таким же именем. Если комната уже существует,
    /// метод возвращает ошибку.
    ///
    /// # Параметры
    ///
    /// - `room`: `SmartRoom` — Экземпляр умной комнаты, который нужно
    ///   добавить в дом.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает:
    /// - `Ok(())` — Если комната успешно добавлена.
    /// - `Err(Box<dyn Error>)` — Если комната с таким же именем
    ///   уже существует.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use lesson_4::smart::location::{SmartHouse, SmartRoom};
    /// let mut smart_house = SmartHouse::new(String::from("My Smart Home"));
    /// let room1 = SmartRoom::new(String::from("Living Room"));
    /// let room2 = SmartRoom::new(String::from("Living Room"));
    ///
    /// // Добавление первой комнаты
    /// match smart_house.add(room1) {
    ///     Ok(_) => println!("Room added successfully!"),
    ///     Err(e) => println!("Error adding room: {}", e),
    /// }
    ///
    /// // Попытка добавить комнату с тем же именем
    /// match smart_house.add(room2) {
    ///     Ok(_) => println!("Room added successfully!"),
    ///     Err(e) => println!("Error adding room: {}", e),
    /// }
    /// ```
    pub fn add(&mut self, room: SmartRoom) -> Result<(), Box<dyn Error>> {
        match self.get_rooms().iter().find(|&v| v.name() == room.name()) {
            Some(_) => Err(format!("room {} already constructed", room.name()).into()),
            None => {
                self.rooms.push(room);

                Ok(())
            }
        }
    }

    #[allow(dead_code)]
    pub fn del(&mut self, room: &str) {
        if let Some(index) = self.get_rooms().iter().position(|r| r.name() == room) {
            self.rooms.remove(index);
        }
    }

    /// Возвращает срез всех умных комнат в доме.
    ///
    /// Этот метод предоставляет доступ к вектору комнат, которые находятся в
    /// экземпляре `SmartHouse`. Он возвращает ссылку на срез `&[SmartRoom]`,
    /// позволяя получить список всех комнат, не изменяя внутреннее состояние
    /// структуры.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает срез `&[SmartRoom]`, который содержит все комнаты,
    /// добавленные в умный дом. Пользователь может итерироваться по этим
    /// комнатам или использовать их для других целей, но не может вносить
    /// изменения в сами комнаты через этот метод.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use lesson_4::smart::location::{SmartHouse, SmartRoom};
    /// let mut smart_house = SmartHouse::new(String::from("My Smart Home"));
    /// smart_house.add(SmartRoom::new(String::from("Living Room"))).unwrap();
    /// smart_house.add(SmartRoom::new(String::from("Bedroom"))).unwrap();
    ///
    /// // Получение списка комнат
    /// for room in smart_house.get_rooms() {
    ///     println!("Room: {}", room.name());
    /// }
    /// ```
    pub fn get_rooms(&self) -> &[SmartRoom] {
        &self.rooms
    }
    /// Создает отчет на основе заданного типа отчета.
    ///
    /// Этот метод принимает объект, реализующий трейт `Reportable`, и вызывает
    /// метод `make`, чтобы сгенерировать отчет, используя данные из текущего
    /// экземпляра `SmartHouse`. Это позволяет разделить логику создания отчетов
    /// на разные структуры, реализующие данный трейт.
    ///
    /// # Параметры
    ///
    /// - `report`: `T` — Объект, реализующий трейт `Reportable`. Этот объект
    ///   содержит логику для генерации отчета на основе данных умного дома.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает:
    /// - `Ok(String)` — Если отчет успешно создан. Возвращает строку,
    ///   представляющую сгенерированный отчет.
    /// - `Err(Box<dyn Error>)` — Если произошла ошибка в процессе создания отчета.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use std::error::Error;
    /// use lesson_4::smart::report::Reportable;
    /// use lesson_4::smart::location::SmartHouse;
    ///
    /// struct SimpleReport;
    ///
    /// impl Reportable for SimpleReport {
    ///     fn make(&self, smart_house: &SmartHouse) -> Result<String, Box<dyn Error>> {
    ///         Ok(format!("Report for: {}", smart_house.name()))
    ///     }
    /// }
    ///
    /// let smart_house = SmartHouse::new(String::from("My Smart Home"));
    /// let report = SimpleReport;
    ///
    /// match smart_house.create_report(report) {
    ///     Ok(report_text) => println!("{}", report_text),
    ///     Err(e) => println!("Error creating report: {}", e),
    /// }
    /// ```
    pub fn create_report<T: Reportable>(&self, report: T) -> Result<String, Box<dyn Error>> {
        report.make(self)
    }
}

impl PartialEq for SmartRoom {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl fmt::Display for SmartHouse {
    /// Форматирует представление умного дома для вывода в строку.
    ///
    /// Этот метод реализует трейты `fmt::Display`, позволяя
    /// выводить экземпляр `SmartHouse` в удобочитаемом формате.
    /// Метод записывает имя умного дома в строковом формате,
    /// предшествуемое стрелкой (`->`).
    ///
    /// # Параметры
    ///
    /// - `f`: `&mut std::fmt::Formatter` — Мутируемая ссылка на форматтер,
    ///   который используется для записи форматированной строки.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает:
    /// - `std::fmt::Result` — Результат выполнения форматирования.
    ///   Если форматирование прошло успешно, возвращается `Ok(())`,
    ///   в противном случае возвращается ошибка формата.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use lesson_4::smart::location::SmartHouse;
    /// let smart_house = SmartHouse::new(String::from("My Smart Home"));
    /// println!("{}", smart_house);
    /// ```
    ///
    /// Выход:
    /// ```plaintext
    /// -> House: My Smart Home
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-> House: {}", self.name())
    }
}

/// Структура `SmartRoom` представляет собой умную комнату,
/// которая содержит название и коллекцию устройств.
///
/// # Поля
///
/// - `name`: строка, представляющая название комнаты.
/// - `devices`: вектор, содержащий устройства, которые могут быть подключены
///   к этой комнате. Устройства представлены через указатели `Arc` на динамические
///   трейт-объекты `Pluggable`.
///
/// ```
#[derive(Clone)]
pub struct SmartRoom {
    name: String,
    devices: Vec<Arc<dyn Pluggable>>,
}

impl SmartRoom {
    /// Создает новый экземпляр `SmartRoom`.
    ///
    /// Этот метод предоставляет возможность инициализировать `SmartRoom`
    /// с заданным названием и пустым списком устройств.
    ///
    /// # Аргументы
    ///
    /// - `name`: Строка, представляющая название новой комнаты.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает новый экземпляр `SmartRoom` с указанным именем и
    /// пустым вектором устройств.
    ///
    /// # Пример
    ///
    /// ```
    /// use lesson_4::smart::location::SmartRoom;
    /// let room = SmartRoom::new(String::from("Bedroom"));
    /// assert_eq!(room.name(), "Bedroom");
    /// assert!(room.devices().is_empty());
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: Vec::default(),
        }
    }
    /// Подключает устройство к комнате.
    ///
    /// Этот метод пытается добавить устройство в список устройств,
    /// подключенных к `SmartRoom`. Если устройство с таким же именем
    /// уже подключено, метод вернет ошибку.
    ///
    /// # Аргументы
    ///
    /// - `device`: Указатель `Arc` на объект, реализующий трейт
    ///   `Pluggable`, который необходимо подключить.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает `Result<(), Box<dyn Error>>`, где:
    /// - `Ok(())` означает успешное подключение устройства.
    /// - `Err(err)` содержит информацию об ошибке, если устройство
    ///   с заданным именем уже подключено.
    ///
    /// # Пример
    ///
    /// ```
    /// use std::sync::Arc;
    /// use lesson_4::smart::location::SmartRoom;
    /// use lesson_4::smart::device::SmartSocket;
    /// let mut room = SmartRoom::new(String::from("Office"));
    /// let device = Arc::new(SmartSocket::new(String::from("Printer")));
    ///
    /// // Подключаем устройство
    /// assert!(room.plug(device.clone()).is_ok());
    ///
    /// // Пытаемся подключить то же устройство снова
    /// assert!(room.plug(device).is_err());
    /// ```
    pub fn plug(&mut self, device: Arc<dyn Pluggable>) -> Result<(), Box<dyn Error>> {
        match &self.devices.iter().find(|&d| d.name() == device.name()) {
            Some(_) => Err(format!("Device with name {} already pluged", device.name()).into()),
            None => {
                self.devices.push(device);
                Ok(())
            }
        }
    }

    #[allow(dead_code)]
    pub fn unplug(&mut self, device: &str) {
        if let Some(index) = self.devices.iter().position(|d| d.name() == device) {
            self.devices.remove(index);
        }
    }

    /// Проверяет, подключено ли устройство к комнате.
    ///
    /// Этот метод принимает ссылку на устройство и проверяет,
    /// существует ли устройство с таким же именем в списке подключенных
    /// устройств в `SmartRoom`.
    ///
    /// # Аргументы
    ///
    /// - `device`: Ссылка на объект, реализующий трейт `Pluggable`,
    ///   которое требуется проверить на подключение.
    ///
    /// # Возвращаемое значение
    ///
    /// Метод возвращает `true`, если устройство подключено к комнате,
    /// и `false` в противном случае.
    ///
    /// # Пример
    ///
    /// ```
    /// use std::sync::Arc;
    /// use lesson_4::smart::location::SmartRoom;
    /// use lesson_4::smart::device::SmartSocket;
    ///
    /// let mut room = SmartRoom::new(String::from("Kitchen"));
    /// let device1 = Arc::new(SmartSocket::new(String::from("Toaster")));
    /// let device2 = Arc::new(SmartSocket::new(String::from("Mixer")));
    ///
    /// room.plug(device1.clone()).unwrap();
    ///
    /// assert!(room.is_connected(&*device1)); // Проверка, что устройство подключено
    /// assert!(!room.is_connected(&*device2)); // Проверка, что устройство не подключено
    /// ```
    pub fn is_connected(&self, device: &dyn Pluggable) -> bool {
        self.devices.iter().any(|d| d.name() == device.name())
    }

    /// Возвращает список имен подключенных устройств.
    ///
    /// Этот метод собирает имена всех устройств, подключенных к
    /// `SmartRoom`, и возвращает их в виде вектора строк.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает `Vec<String>`, содержащий имена всех подключенных
    /// устройств. Если устройств нет, будет возвращен пустой вектор.
    ///
    /// # Пример
    ///
    /// ```
    /// use std::sync::Arc;
    /// use lesson_4::smart::location::SmartRoom;
    /// use lesson_4::smart::device::SmartSocket;
    ///
    /// let mut room = SmartRoom::new(String::from("Living Room"));
    /// let device1 = Arc::new(SmartSocket::new(String::from("TV")));
    /// let device2 = Arc::new(SmartSocket::new(String::from("Sound System")));
    ///
    /// room.plug(device1.clone()).unwrap();
    /// room.plug(device2.clone()).unwrap();
    ///
    /// let devices = room.devices();
    /// assert_eq!(devices, vec!["TV".to_string(), "Sound System".to_string()]);
    ///
    /// // Проверка пустого списка устройств, если комната еще не содержит устройств.
    /// let empty_room = SmartRoom::new(String::from("Empty Room"));
    /// assert!(empty_room.devices().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn devices(&self) -> Vec<String> {
        self.devices.iter().map(|d| d.name().to_string()).collect()
    }

    /// Возвращает имя устройства.
    ///
    /// Этот метод предоставляет доступ к имени устройства,
    /// хранящемуся в поле структуры `Device`.
    ///
    /// # Возвращаемое значение
    ///
    /// Взвращает ссылку на строку с именем устройства `&str`.
    /// Имя устройства будет неизменяемым.
    ///
    /// # Пример
    ///
    /// ```
    /// use lesson_4::smart::location::SmartRoom;
    /// let room = SmartRoom::new(String::from("Ketchen"));
    /// assert_eq!(room.name(), "Ketchen");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for SmartRoom {
    /// Форматирует вывод информации о комнате.
    ///
    /// Этот метод реализует трейт `fmt::Display`, позволяя
    /// выводить структуру `SmartRoom` в читаемом виде.
    /// В данном случае отображается имя комнаты.
    ///
    /// # Аргументы
    ///
    /// - `f`: Ссылка на объект `Formatter`, который
    ///   используется для записи форматированного вывода.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает `fmt::Result`, который указывает на успешность
    /// операции форматирования. В случае успешного выполнения
    /// будет возвращено `Ok(())`, в пративном случае может быть
    /// возвращена ошибка, описывающая проблему.
    ///
    /// # Пример использования
    ///
    /// ```
    /// use lesson_4::smart::location::SmartRoom;
    /// let room = SmartRoom::new(String::from("Living Room"));
    /// println!("{}", room); // Выведет: --> Room: Living Room
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--> Room: {}", self.name())
    }
}
