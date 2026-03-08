# NES
Rustで記述されたNESエミュレーターです。  
Rustの言語仕様, エミュレーターの動作原理, Gitの使い方,... の学習を目的としています。

## 環境
- Windows11 WSL2(WSLg)
- Docker Desktop
- VSCode

## ロードマップ
### Phase 1
- [ ] CPU, 命令セット, NROMマッパーの実装
- [ ] .nesファイルのリーダーを実装
- [ ] [nestest](https://github.com/nwidger/nintengo/blob/master/m65go2/test-roms/nestest)をパスする

### Phase 2
- [ ] PPUの実装
- [ ] [Hello, World!](https://tekepen.com/nes/sample.html)の出力

### Phase 3
- [ ] ゲームパッド, APU, マッパーなどを実装
- [ ] GUIを実装
- [ ] [nesdev/Emulator Tests](https://www.nesdev.org/wiki/Emulator_tests)を一通りパスする
- [ ] スーパーマリオブラザーズをプレイする
