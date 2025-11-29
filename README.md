### Disclaimer: 
# This content is not affiliated, approved, sponsored or approved specifically by Supercell and Supercell is not responsible for it. For more, see the Supercell Fan Content Policy: www.supercell.com/fan-content-policy

### Overview
1. Search by bytes (Press alt + B):
   ```
   1A D5 00 00 00 00 00 00
   ```
   
   ![A](search.png)
   
   These bytes always come after the OBF key
   
3. Сopy the key
   
![O](hex.png)

The key in this: 
```
F25BE346CF65BC6994892E7DB84B7696E7E2190003152CB16A4EDB3379377824D500F41A5CCF96C15FFAFF9572A6746FEF8DCEA04AA49DC221D11744DA783597EC1D3578271517D185419DA4E470CE8D8EE974A61D41FFFA61A296CFBE87F40045A37837E8CADB4EB2392C15DB3219E2F1DF764BD9432E896F51BC65CA58E35B
```

---

### Using the Obfuscated Key:
## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/FMZNkdv/ScKey.git
   cd ScKey
   ```

2. Install Java:
   
  **Arch Linux:**
  ```bash
  sudo pacman -S jdk-openjdk
  ```
  
  **Ubuntu/Debian:**
  ```bash
  sudo apt install default-jdk
  ```
  
  **Fedora:**
  ```bash
  sudo dnf install java-17-openjdk-devel
  ```

3. Run Jar with the received key:
   ```bash
   java -jar deobf.jar F25BE346CF65BC6994892E7DB84B7696E7E2190003152CB16A4EDB3379377824D500F41A5CCF96C15FFAFF9572A6746FEF8DCEA04AA49DC221D11744DA783597EC1D3578271517D185419DA4E470CE8D8EE974A61D41FFFA61A296CFBE87F40045A37837E8CADB4EB2392C15DB3219E2F1DF764BD9432E896F51BC65CA58E35B
   ```

Done! You've got the SPK.

# Warnings!!!!
## You need to use Lib with the already killed promon
