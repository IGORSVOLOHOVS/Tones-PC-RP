### 1. Инициализация проекта

- [ ] **Цели проекта**
  - [ ] **Каковы главные цели проекта:**
  - The task is to create the dependencies from Prisma database connector and Rust to call the functions by protocol.
  - [ ] **Какие проблемы проект должен решить:**
  - The project have to call needed functions, user want to see the result on microcontroller RP 4
  - [ ] **Кто целевые пользователи:**
  - Developers.
  - [ ] **Какие у пользователей есть потребности и ожидания:**
  - User want to get the date from the set of protocols, parse it and call needed function.
  - [ ] **Каковы критерии успешного завершения проекта:**
  - Result.
- [ ] **Методология управления проектом:** ToDo, Kanban

### 2. Сбор требований

- [ ] **Бизнес требования**
- Eto deremo dolzno rabotatj stobi himika ne grohnutj. 
  - [ ] **Данные на входе:**
  - schema.prisma
  - *.sql with the table of prtocols and etc
  - [ ] **Данные на выходе:**
  - Rust function which read the db using prisma
  - Rust function which receive protocols(in json format)
  - Rust function which find and parse to structure protocol
  - Rust function which is a loop for protocol running
- [ ] **Функциональные требования**
  - [ ] **Какие ключевые функции должен иметь продукт:**
  - [ ] **Есть ли специальные функциональные требования:**
- [ ] **Нефункциональные требования**
  - [ ] **Какие требования к производительности:**
  - [ ] **Какие требования к безопасности и приватности:**
  - [ ] **Требования к совместимости и интеграции:**
- [ ] **Ограничения**
  - [ ] **Какие существуют технические ограничения:**
  - [ ] **Какие временные и бюджетные ограничения:**

### 3. Разработка спецификаций

- [ ] **Use-case**
  - [ ] **Список сущностей:**
  - [ ] **Основные сценарии:** Mermaid, PlantUML, Draw.io

### 4. Проектирование системы

- [ ] **Архитектурная схема:** Микросервисная архитектура, сервисы(.h.dll)
- [ ] **Диаграмма классов:** UML, Algorithm View,
- [ ] **Паттерны проекирования**
- [ ] **Технологический стек:** C/C++ WASM для бэкэнда, MySQL для базы данных, HTML для визуализации сайта

### 5. Разработка:

- [ ] **Кодирование:** C/C++
  - [ ] **Добавление исходных файлов (сервисов .h)**
    - [ ] **Каждому классу соответствует cpp**
  - [ ] **Реализация основных классов (функций) + тестирование + документация**
    - [ ] **double,string,int,vector,циклы**
    - [ ] **Комментарии**
    - [ ] **if/else/exceptions,noexcept**
  - [ ] **Оптимизация + тестирование + бенчмарк:**
    - [ ] **Умные указатели и ссылки**
    - [ ] **Сжатие данных**
    - [ ] **Unordered контейнеры**
    - [ ] **Алгоритмы**
  - [ ] **Парралельное программирование + бенчмарк**
    - [ ] **future/task/semaphors**
  - [ ] **Метапрограммирование + бенчмарк + тестирование**
    - [ ] **constexpr**
    - [ ] **template/requires/concept**
- [ ] **Документирование** Doxygen, License
- [ ] **Логирование**

### 6. Тестирование

- [ ] **Ограничения параметров**
- [ ] **GTest**
- [ ] **Timers/Benchmark**
- [ ] **Random word generator**
- [ ] **Use-case**

### 7. Интеграция и развертывание

- [X] **Сборка:** CMake, Package managers
- [ ] **Развертывание:** Docker, Linux
- [ ] **Интеграция:** Python, C#, WASM

### 8. Поддержка и обслуживание

- [X] **Контроль версий:** github
- [ ] **Обновления:** github ci/cd
- [ ] **Архивация:** CPack, NSIS
