#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JPEG codec configuration register 0"]
    pub jpeg_confr0: crate::Reg<jpeg_confr0::JPEG_CONFR0_SPEC>,
    #[doc = "0x04 - JPEG codec configuration register 1"]
    pub jpeg_confr1: crate::Reg<jpeg_confr1::JPEG_CONFR1_SPEC>,
    #[doc = "0x08 - JPEG codec configuration register 2"]
    pub jpeg_confr2: crate::Reg<jpeg_confr2::JPEG_CONFR2_SPEC>,
    #[doc = "0x0c - JPEG codec configuration register 3"]
    pub jpeg_confr3: crate::Reg<jpeg_confr3::JPEG_CONFR3_SPEC>,
    #[doc = "0x10 - JPEG codec configuration register 4"]
    pub jpeg_confr4: crate::Reg<jpeg_confr4::JPEG_CONFR4_SPEC>,
    #[doc = "0x14 - JPEG codec configuration register 5"]
    pub jpeg_confr5: crate::Reg<jpeg_confr5::JPEG_CONFR5_SPEC>,
    #[doc = "0x18 - JPEG codec configuration register 6"]
    pub jpeg_confr6: crate::Reg<jpeg_confr6::JPEG_CONFR6_SPEC>,
    #[doc = "0x1c - JPEG codec configuration register 7"]
    pub jpeg_confr7: crate::Reg<jpeg_confr7::JPEG_CONFR7_SPEC>,
    _reserved8: [u8; 0x10],
    #[doc = "0x30 - JPEG control register"]
    pub jpeg_cr: crate::Reg<jpeg_cr::JPEG_CR_SPEC>,
    #[doc = "0x34 - JPEG status register"]
    pub jpeg_sr: crate::Reg<jpeg_sr::JPEG_SR_SPEC>,
    #[doc = "0x38 - JPEG clear flag register"]
    pub jpeg_cfr: crate::Reg<jpeg_cfr::JPEG_CFR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x40 - JPEG data input register"]
    pub jpeg_dir: crate::Reg<jpeg_dir::JPEG_DIR_SPEC>,
    #[doc = "0x44 - JPEG data output register"]
    pub jpeg_dor: crate::Reg<jpeg_dor::JPEG_DOR_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x50 - JPEG quantization tables"]
    pub qmem0_0: crate::Reg<qmem0_0::QMEM0_0_SPEC>,
    #[doc = "0x54 - JPEG quantization tables"]
    pub qmem0_1: crate::Reg<qmem0_1::QMEM0_1_SPEC>,
    #[doc = "0x58 - JPEG quantization tables"]
    pub qmem0_2: crate::Reg<qmem0_2::QMEM0_2_SPEC>,
    #[doc = "0x5c - JPEG quantization tables"]
    pub qmem0_3: crate::Reg<qmem0_3::QMEM0_3_SPEC>,
    #[doc = "0x60 - JPEG quantization tables"]
    pub qmem0_4: crate::Reg<qmem0_4::QMEM0_4_SPEC>,
    #[doc = "0x64 - JPEG quantization tables"]
    pub qmem0_5: crate::Reg<qmem0_5::QMEM0_5_SPEC>,
    #[doc = "0x68 - JPEG quantization tables"]
    pub qmem0_6: crate::Reg<qmem0_6::QMEM0_6_SPEC>,
    #[doc = "0x6c - JPEG quantization tables"]
    pub qmem0_7: crate::Reg<qmem0_7::QMEM0_7_SPEC>,
    #[doc = "0x70 - JPEG quantization tables"]
    pub qmem0_8: crate::Reg<qmem0_8::QMEM0_8_SPEC>,
    #[doc = "0x74 - JPEG quantization tables"]
    pub qmem0_9: crate::Reg<qmem0_9::QMEM0_9_SPEC>,
    #[doc = "0x78 - JPEG quantization tables"]
    pub qmem0_10: crate::Reg<qmem0_10::QMEM0_10_SPEC>,
    #[doc = "0x7c - JPEG quantization tables"]
    pub qmem0_11: crate::Reg<qmem0_11::QMEM0_11_SPEC>,
    #[doc = "0x80 - JPEG quantization tables"]
    pub qmem0_12: crate::Reg<qmem0_12::QMEM0_12_SPEC>,
    #[doc = "0x84 - JPEG quantization tables"]
    pub qmem0_13: crate::Reg<qmem0_13::QMEM0_13_SPEC>,
    #[doc = "0x88 - JPEG quantization tables"]
    pub qmem0_14: crate::Reg<qmem0_14::QMEM0_14_SPEC>,
    #[doc = "0x8c - JPEG quantization tables"]
    pub qmem0_15: crate::Reg<qmem0_15::QMEM0_15_SPEC>,
    #[doc = "0x90 - JPEG quantization tables"]
    pub qmem1_0: crate::Reg<qmem1_0::QMEM1_0_SPEC>,
    #[doc = "0x94 - JPEG quantization tables"]
    pub qmem1_1: crate::Reg<qmem1_1::QMEM1_1_SPEC>,
    #[doc = "0x98 - JPEG quantization tables"]
    pub qmem1_2: crate::Reg<qmem1_2::QMEM1_2_SPEC>,
    #[doc = "0x9c - JPEG quantization tables"]
    pub qmem1_3: crate::Reg<qmem1_3::QMEM1_3_SPEC>,
    #[doc = "0xa0 - JPEG quantization tables"]
    pub qmem1_4: crate::Reg<qmem1_4::QMEM1_4_SPEC>,
    #[doc = "0xa4 - JPEG quantization tables"]
    pub qmem1_5: crate::Reg<qmem1_5::QMEM1_5_SPEC>,
    #[doc = "0xa8 - JPEG quantization tables"]
    pub qmem1_6: crate::Reg<qmem1_6::QMEM1_6_SPEC>,
    #[doc = "0xac - JPEG quantization tables"]
    pub qmem1_7: crate::Reg<qmem1_7::QMEM1_7_SPEC>,
    #[doc = "0xb0 - JPEG quantization tables"]
    pub qmem1_8: crate::Reg<qmem1_8::QMEM1_8_SPEC>,
    #[doc = "0xb4 - JPEG quantization tables"]
    pub qmem1_9: crate::Reg<qmem1_9::QMEM1_9_SPEC>,
    #[doc = "0xb8 - JPEG quantization tables"]
    pub qmem1_10: crate::Reg<qmem1_10::QMEM1_10_SPEC>,
    #[doc = "0xbc - JPEG quantization tables"]
    pub qmem1_11: crate::Reg<qmem1_11::QMEM1_11_SPEC>,
    #[doc = "0xc0 - JPEG quantization tables"]
    pub qmem1_12: crate::Reg<qmem1_12::QMEM1_12_SPEC>,
    #[doc = "0xc4 - JPEG quantization tables"]
    pub qmem1_13: crate::Reg<qmem1_13::QMEM1_13_SPEC>,
    #[doc = "0xc8 - JPEG quantization tables"]
    pub qmem1_14: crate::Reg<qmem1_14::QMEM1_14_SPEC>,
    #[doc = "0xcc - JPEG quantization tables"]
    pub qmem1_15: crate::Reg<qmem1_15::QMEM1_15_SPEC>,
    #[doc = "0xd0 - JPEG quantization tables"]
    pub qmem2_0: crate::Reg<qmem2_0::QMEM2_0_SPEC>,
    #[doc = "0xd4 - JPEG quantization tables"]
    pub qmem2_1: crate::Reg<qmem2_1::QMEM2_1_SPEC>,
    #[doc = "0xd8 - JPEG quantization tables"]
    pub qmem2_2: crate::Reg<qmem2_2::QMEM2_2_SPEC>,
    #[doc = "0xdc - JPEG quantization tables"]
    pub qmem2_3: crate::Reg<qmem2_3::QMEM2_3_SPEC>,
    #[doc = "0xe0 - JPEG quantization tables"]
    pub qmem2_4: crate::Reg<qmem2_4::QMEM2_4_SPEC>,
    #[doc = "0xe4 - JPEG quantization tables"]
    pub qmem2_5: crate::Reg<qmem2_5::QMEM2_5_SPEC>,
    #[doc = "0xe8 - JPEG quantization tables"]
    pub qmem2_6: crate::Reg<qmem2_6::QMEM2_6_SPEC>,
    #[doc = "0xec - JPEG quantization tables"]
    pub qmem2_7: crate::Reg<qmem2_7::QMEM2_7_SPEC>,
    #[doc = "0xf0 - JPEG quantization tables"]
    pub qmem2_8: crate::Reg<qmem2_8::QMEM2_8_SPEC>,
    #[doc = "0xf4 - JPEG quantization tables"]
    pub qmem2_9: crate::Reg<qmem2_9::QMEM2_9_SPEC>,
    #[doc = "0xf8 - JPEG quantization tables"]
    pub qmem2_10: crate::Reg<qmem2_10::QMEM2_10_SPEC>,
    #[doc = "0xfc - JPEG quantization tables"]
    pub qmem2_11: crate::Reg<qmem2_11::QMEM2_11_SPEC>,
    #[doc = "0x100 - JPEG quantization tables"]
    pub qmem2_12: crate::Reg<qmem2_12::QMEM2_12_SPEC>,
    #[doc = "0x104 - JPEG quantization tables"]
    pub qmem2_13: crate::Reg<qmem2_13::QMEM2_13_SPEC>,
    #[doc = "0x108 - JPEG quantization tables"]
    pub qmem2_14: crate::Reg<qmem2_14::QMEM2_14_SPEC>,
    #[doc = "0x10c - JPEG quantization tables"]
    pub qmem2_15: crate::Reg<qmem2_15::QMEM2_15_SPEC>,
    #[doc = "0x110 - JPEG quantization tables"]
    pub qmem3_0: crate::Reg<qmem3_0::QMEM3_0_SPEC>,
    #[doc = "0x114 - JPEG quantization tables"]
    pub qmem3_1: crate::Reg<qmem3_1::QMEM3_1_SPEC>,
    #[doc = "0x118 - JPEG quantization tables"]
    pub qmem3_2: crate::Reg<qmem3_2::QMEM3_2_SPEC>,
    #[doc = "0x11c - JPEG quantization tables"]
    pub qmem3_3: crate::Reg<qmem3_3::QMEM3_3_SPEC>,
    #[doc = "0x120 - JPEG quantization tables"]
    pub qmem3_4: crate::Reg<qmem3_4::QMEM3_4_SPEC>,
    #[doc = "0x124 - JPEG quantization tables"]
    pub qmem3_5: crate::Reg<qmem3_5::QMEM3_5_SPEC>,
    #[doc = "0x128 - JPEG quantization tables"]
    pub qmem3_6: crate::Reg<qmem3_6::QMEM3_6_SPEC>,
    #[doc = "0x12c - JPEG quantization tables"]
    pub qmem3_7: crate::Reg<qmem3_7::QMEM3_7_SPEC>,
    #[doc = "0x130 - JPEG quantization tables"]
    pub qmem3_8: crate::Reg<qmem3_8::QMEM3_8_SPEC>,
    #[doc = "0x134 - JPEG quantization tables"]
    pub qmem3_9: crate::Reg<qmem3_9::QMEM3_9_SPEC>,
    #[doc = "0x138 - JPEG quantization tables"]
    pub qmem3_10: crate::Reg<qmem3_10::QMEM3_10_SPEC>,
    #[doc = "0x13c - JPEG quantization tables"]
    pub qmem3_11: crate::Reg<qmem3_11::QMEM3_11_SPEC>,
    #[doc = "0x140 - JPEG quantization tables"]
    pub qmem3_12: crate::Reg<qmem3_12::QMEM3_12_SPEC>,
    #[doc = "0x144 - JPEG quantization tables"]
    pub qmem3_13: crate::Reg<qmem3_13::QMEM3_13_SPEC>,
    #[doc = "0x148 - JPEG quantization tables"]
    pub qmem3_14: crate::Reg<qmem3_14::QMEM3_14_SPEC>,
    #[doc = "0x14c - JPEG quantization tables"]
    pub qmem3_15: crate::Reg<qmem3_15::QMEM3_15_SPEC>,
    #[doc = "0x150 - JPEG HuffMin tables"]
    pub huffmin_0: crate::Reg<huffmin_0::HUFFMIN_0_SPEC>,
    #[doc = "0x154 - JPEG HuffMin tables"]
    pub huffmin_1: crate::Reg<huffmin_1::HUFFMIN_1_SPEC>,
    #[doc = "0x158 - JPEG HuffMin tables"]
    pub huffmin_2: crate::Reg<huffmin_2::HUFFMIN_2_SPEC>,
    #[doc = "0x15c - JPEG HuffMin tables"]
    pub huffmin_3: crate::Reg<huffmin_3::HUFFMIN_3_SPEC>,
    #[doc = "0x160 - JPEG HuffMin tables"]
    pub huffmin_4: crate::Reg<huffmin_4::HUFFMIN_4_SPEC>,
    #[doc = "0x164 - JPEG HuffMin tables"]
    pub huffmin_5: crate::Reg<huffmin_5::HUFFMIN_5_SPEC>,
    #[doc = "0x168 - JPEG HuffMin tables"]
    pub huffmin_6: crate::Reg<huffmin_6::HUFFMIN_6_SPEC>,
    #[doc = "0x16c - JPEG HuffMin tables"]
    pub huffmin_7: crate::Reg<huffmin_7::HUFFMIN_7_SPEC>,
    #[doc = "0x170 - JPEG HuffMin tables"]
    pub huffmin_8: crate::Reg<huffmin_8::HUFFMIN_8_SPEC>,
    #[doc = "0x174 - JPEG HuffMin tables"]
    pub huffmin_9: crate::Reg<huffmin_9::HUFFMIN_9_SPEC>,
    #[doc = "0x178 - JPEG HuffMin tables"]
    pub huffmin_10: crate::Reg<huffmin_10::HUFFMIN_10_SPEC>,
    #[doc = "0x17c - JPEG HuffMin tables"]
    pub huffmin_11: crate::Reg<huffmin_11::HUFFMIN_11_SPEC>,
    #[doc = "0x180 - JPEG HuffMin tables"]
    pub huffmin_12: crate::Reg<huffmin_12::HUFFMIN_12_SPEC>,
    #[doc = "0x184 - JPEG HuffMin tables"]
    pub huffmin_13: crate::Reg<huffmin_13::HUFFMIN_13_SPEC>,
    #[doc = "0x188 - JPEG HuffMin tables"]
    pub huffmin_14: crate::Reg<huffmin_14::HUFFMIN_14_SPEC>,
    #[doc = "0x18c - JPEG HuffMin tables"]
    pub huffmin_15: crate::Reg<huffmin_15::HUFFMIN_15_SPEC>,
    #[doc = "0x190 - JPEG HuffSymb tables"]
    pub huffbase0: crate::Reg<huffbase0::HUFFBASE0_SPEC>,
    #[doc = "0x194 - JPEG HuffSymb tables"]
    pub huffbase1: crate::Reg<huffbase1::HUFFBASE1_SPEC>,
    #[doc = "0x198 - JPEG HuffSymb tables"]
    pub huffbase2: crate::Reg<huffbase2::HUFFBASE2_SPEC>,
    #[doc = "0x19c - JPEG HuffSymb tables"]
    pub huffbase3: crate::Reg<huffbase3::HUFFBASE3_SPEC>,
    #[doc = "0x1a0 - JPEG HuffSymb tables"]
    pub huffbase4: crate::Reg<huffbase4::HUFFBASE4_SPEC>,
    #[doc = "0x1a4 - JPEG HuffSymb tables"]
    pub huffbase5: crate::Reg<huffbase5::HUFFBASE5_SPEC>,
    #[doc = "0x1a8 - JPEG HuffSymb tables"]
    pub huffbase6: crate::Reg<huffbase6::HUFFBASE6_SPEC>,
    #[doc = "0x1ac - JPEG HuffSymb tables"]
    pub huffbase7: crate::Reg<huffbase7::HUFFBASE7_SPEC>,
    #[doc = "0x1b0 - JPEG HuffSymb tables"]
    pub huffbase8: crate::Reg<huffbase8::HUFFBASE8_SPEC>,
    #[doc = "0x1b4 - JPEG HuffSymb tables"]
    pub huffbase9: crate::Reg<huffbase9::HUFFBASE9_SPEC>,
    #[doc = "0x1b8 - JPEG HuffSymb tables"]
    pub huffbase10: crate::Reg<huffbase10::HUFFBASE10_SPEC>,
    #[doc = "0x1bc - JPEG HuffSymb tables"]
    pub huffbase11: crate::Reg<huffbase11::HUFFBASE11_SPEC>,
    #[doc = "0x1c0 - JPEG HuffSymb tables"]
    pub huffbase12: crate::Reg<huffbase12::HUFFBASE12_SPEC>,
    #[doc = "0x1c4 - JPEG HuffSymb tables"]
    pub huffbase13: crate::Reg<huffbase13::HUFFBASE13_SPEC>,
    #[doc = "0x1c8 - JPEG HuffSymb tables"]
    pub huffbase14: crate::Reg<huffbase14::HUFFBASE14_SPEC>,
    #[doc = "0x1cc - JPEG HuffSymb tables"]
    pub huffbase15: crate::Reg<huffbase15::HUFFBASE15_SPEC>,
    #[doc = "0x1d0 - JPEG HuffSymb tables"]
    pub huffbase16: crate::Reg<huffbase16::HUFFBASE16_SPEC>,
    #[doc = "0x1d4 - JPEG HuffSymb tables"]
    pub huffbase17: crate::Reg<huffbase17::HUFFBASE17_SPEC>,
    #[doc = "0x1d8 - JPEG HuffSymb tables"]
    pub huffbase18: crate::Reg<huffbase18::HUFFBASE18_SPEC>,
    #[doc = "0x1dc - JPEG HuffSymb tables"]
    pub huffbase19: crate::Reg<huffbase19::HUFFBASE19_SPEC>,
    #[doc = "0x1e0 - JPEG HuffSymb tables"]
    pub huffbase20: crate::Reg<huffbase20::HUFFBASE20_SPEC>,
    #[doc = "0x1e4 - JPEG HuffSymb tables"]
    pub huffbase21: crate::Reg<huffbase21::HUFFBASE21_SPEC>,
    #[doc = "0x1e8 - JPEG HuffSymb tables"]
    pub huffbase22: crate::Reg<huffbase22::HUFFBASE22_SPEC>,
    #[doc = "0x1ec - JPEG HuffSymb tables"]
    pub huffbase23: crate::Reg<huffbase23::HUFFBASE23_SPEC>,
    #[doc = "0x1f0 - JPEG HuffSymb tables"]
    pub huffbase24: crate::Reg<huffbase24::HUFFBASE24_SPEC>,
    #[doc = "0x1f4 - JPEG HuffSymb tables"]
    pub huffbase25: crate::Reg<huffbase25::HUFFBASE25_SPEC>,
    #[doc = "0x1f8 - JPEG HuffSymb tables"]
    pub huffbase26: crate::Reg<huffbase26::HUFFBASE26_SPEC>,
    #[doc = "0x1fc - JPEG HuffSymb tables"]
    pub huffbase27: crate::Reg<huffbase27::HUFFBASE27_SPEC>,
    #[doc = "0x200 - JPEG HuffSymb tables"]
    pub huffbase28: crate::Reg<huffbase28::HUFFBASE28_SPEC>,
    #[doc = "0x204 - JPEG HuffSymb tables"]
    pub huffbase29: crate::Reg<huffbase29::HUFFBASE29_SPEC>,
    #[doc = "0x208 - JPEG HuffSymb tables"]
    pub huffbase30: crate::Reg<huffbase30::HUFFBASE30_SPEC>,
    #[doc = "0x20c - JPEG HuffSymb tables"]
    pub huffbase31: crate::Reg<huffbase31::HUFFBASE31_SPEC>,
    #[doc = "0x210 - JPEG HUFFSYMB tables"]
    pub huffsymb0: crate::Reg<huffsymb0::HUFFSYMB0_SPEC>,
    #[doc = "0x214 - JPEG HUFFSYMB tables"]
    pub huffsymb1: crate::Reg<huffsymb1::HUFFSYMB1_SPEC>,
    #[doc = "0x218 - JPEG HUFFSYMB tables"]
    pub huffsymb2: crate::Reg<huffsymb2::HUFFSYMB2_SPEC>,
    #[doc = "0x21c - JPEG HUFFSYMB tables"]
    pub huffsymb3: crate::Reg<huffsymb3::HUFFSYMB3_SPEC>,
    #[doc = "0x220 - JPEG HUFFSYMB tables"]
    pub huffsymb4: crate::Reg<huffsymb4::HUFFSYMB4_SPEC>,
    #[doc = "0x224 - JPEG HUFFSYMB tables"]
    pub huffsymb5: crate::Reg<huffsymb5::HUFFSYMB5_SPEC>,
    #[doc = "0x228 - JPEG HUFFSYMB tables"]
    pub huffsymb6: crate::Reg<huffsymb6::HUFFSYMB6_SPEC>,
    #[doc = "0x22c - JPEG HUFFSYMB tables"]
    pub huffsymb7: crate::Reg<huffsymb7::HUFFSYMB7_SPEC>,
    #[doc = "0x230 - JPEG HUFFSYMB tables"]
    pub huffsymb8: crate::Reg<huffsymb8::HUFFSYMB8_SPEC>,
    #[doc = "0x234 - JPEG HUFFSYMB tables"]
    pub huffsymb9: crate::Reg<huffsymb9::HUFFSYMB9_SPEC>,
    #[doc = "0x238 - JPEG HUFFSYMB tables"]
    pub huffsymb10: crate::Reg<huffsymb10::HUFFSYMB10_SPEC>,
    #[doc = "0x23c - JPEG HUFFSYMB tables"]
    pub huffsymb11: crate::Reg<huffsymb11::HUFFSYMB11_SPEC>,
    #[doc = "0x240 - JPEG HUFFSYMB tables"]
    pub huffsymb12: crate::Reg<huffsymb12::HUFFSYMB12_SPEC>,
    #[doc = "0x244 - JPEG HUFFSYMB tables"]
    pub huffsymb13: crate::Reg<huffsymb13::HUFFSYMB13_SPEC>,
    #[doc = "0x248 - JPEG HUFFSYMB tables"]
    pub huffsymb14: crate::Reg<huffsymb14::HUFFSYMB14_SPEC>,
    #[doc = "0x24c - JPEG HUFFSYMB tables"]
    pub huffsymb15: crate::Reg<huffsymb15::HUFFSYMB15_SPEC>,
    #[doc = "0x250 - JPEG HUFFSYMB tables"]
    pub huffsymb16: crate::Reg<huffsymb16::HUFFSYMB16_SPEC>,
    #[doc = "0x254 - JPEG HUFFSYMB tables"]
    pub huffsymb17: crate::Reg<huffsymb17::HUFFSYMB17_SPEC>,
    #[doc = "0x258 - JPEG HUFFSYMB tables"]
    pub huffsymb18: crate::Reg<huffsymb18::HUFFSYMB18_SPEC>,
    #[doc = "0x25c - JPEG HUFFSYMB tables"]
    pub huffsymb19: crate::Reg<huffsymb19::HUFFSYMB19_SPEC>,
    #[doc = "0x260 - JPEG HUFFSYMB tables"]
    pub huffsymb20: crate::Reg<huffsymb20::HUFFSYMB20_SPEC>,
    #[doc = "0x264 - JPEG HUFFSYMB tables"]
    pub huffsymb21: crate::Reg<huffsymb21::HUFFSYMB21_SPEC>,
    #[doc = "0x268 - JPEG HUFFSYMB tables"]
    pub huffsymb22: crate::Reg<huffsymb22::HUFFSYMB22_SPEC>,
    #[doc = "0x26c - JPEG HUFFSYMB tables"]
    pub huffsymb23: crate::Reg<huffsymb23::HUFFSYMB23_SPEC>,
    #[doc = "0x270 - JPEG HUFFSYMB tables"]
    pub huffsymb24: crate::Reg<huffsymb24::HUFFSYMB24_SPEC>,
    #[doc = "0x274 - JPEG HUFFSYMB tables"]
    pub huffsymb25: crate::Reg<huffsymb25::HUFFSYMB25_SPEC>,
    #[doc = "0x278 - JPEG HUFFSYMB tables"]
    pub huffsymb26: crate::Reg<huffsymb26::HUFFSYMB26_SPEC>,
    #[doc = "0x27c - JPEG HUFFSYMB tables"]
    pub huffsymb27: crate::Reg<huffsymb27::HUFFSYMB27_SPEC>,
    #[doc = "0x280 - JPEG HUFFSYMB tables"]
    pub huffsymb28: crate::Reg<huffsymb28::HUFFSYMB28_SPEC>,
    #[doc = "0x284 - JPEG HUFFSYMB tables"]
    pub huffsymb29: crate::Reg<huffsymb29::HUFFSYMB29_SPEC>,
    #[doc = "0x288 - JPEG HUFFSYMB tables"]
    pub huffsymb30: crate::Reg<huffsymb30::HUFFSYMB30_SPEC>,
    #[doc = "0x28c - JPEG HUFFSYMB tables"]
    pub huffsymb31: crate::Reg<huffsymb31::HUFFSYMB31_SPEC>,
    #[doc = "0x290 - JPEG HUFFSYMB tables"]
    pub huffsymb32: crate::Reg<huffsymb32::HUFFSYMB32_SPEC>,
    #[doc = "0x294 - JPEG HUFFSYMB tables"]
    pub huffsymb33: crate::Reg<huffsymb33::HUFFSYMB33_SPEC>,
    #[doc = "0x298 - JPEG HUFFSYMB tables"]
    pub huffsymb34: crate::Reg<huffsymb34::HUFFSYMB34_SPEC>,
    #[doc = "0x29c - JPEG HUFFSYMB tables"]
    pub huffsymb35: crate::Reg<huffsymb35::HUFFSYMB35_SPEC>,
    #[doc = "0x2a0 - JPEG HUFFSYMB tables"]
    pub huffsymb36: crate::Reg<huffsymb36::HUFFSYMB36_SPEC>,
    #[doc = "0x2a4 - JPEG HUFFSYMB tables"]
    pub huffsymb37: crate::Reg<huffsymb37::HUFFSYMB37_SPEC>,
    #[doc = "0x2a8 - JPEG HUFFSYMB tables"]
    pub huffsymb38: crate::Reg<huffsymb38::HUFFSYMB38_SPEC>,
    #[doc = "0x2ac - JPEG HUFFSYMB tables"]
    pub huffsymb39: crate::Reg<huffsymb39::HUFFSYMB39_SPEC>,
    #[doc = "0x2b0 - JPEG HUFFSYMB tables"]
    pub huffsymb40: crate::Reg<huffsymb40::HUFFSYMB40_SPEC>,
    #[doc = "0x2b4 - JPEG HUFFSYMB tables"]
    pub huffsymb41: crate::Reg<huffsymb41::HUFFSYMB41_SPEC>,
    #[doc = "0x2b8 - JPEG HUFFSYMB tables"]
    pub huffsymb42: crate::Reg<huffsymb42::HUFFSYMB42_SPEC>,
    #[doc = "0x2bc - JPEG HUFFSYMB tables"]
    pub huffsymb43: crate::Reg<huffsymb43::HUFFSYMB43_SPEC>,
    #[doc = "0x2c0 - JPEG HUFFSYMB tables"]
    pub huffsymb44: crate::Reg<huffsymb44::HUFFSYMB44_SPEC>,
    #[doc = "0x2c4 - JPEG HUFFSYMB tables"]
    pub huffsymb45: crate::Reg<huffsymb45::HUFFSYMB45_SPEC>,
    #[doc = "0x2c8 - JPEG HUFFSYMB tables"]
    pub huffsymb46: crate::Reg<huffsymb46::HUFFSYMB46_SPEC>,
    #[doc = "0x2cc - JPEG HUFFSYMB tables"]
    pub huffsymb47: crate::Reg<huffsymb47::HUFFSYMB47_SPEC>,
    #[doc = "0x2d0 - JPEG HUFFSYMB tables"]
    pub huffsymb48: crate::Reg<huffsymb48::HUFFSYMB48_SPEC>,
    #[doc = "0x2d4 - JPEG HUFFSYMB tables"]
    pub huffsymb49: crate::Reg<huffsymb49::HUFFSYMB49_SPEC>,
    #[doc = "0x2d8 - JPEG HUFFSYMB tables"]
    pub huffsymb50: crate::Reg<huffsymb50::HUFFSYMB50_SPEC>,
    #[doc = "0x2dc - JPEG HUFFSYMB tables"]
    pub huffsymb51: crate::Reg<huffsymb51::HUFFSYMB51_SPEC>,
    #[doc = "0x2e0 - JPEG HUFFSYMB tables"]
    pub huffsymb52: crate::Reg<huffsymb52::HUFFSYMB52_SPEC>,
    #[doc = "0x2e4 - JPEG HUFFSYMB tables"]
    pub huffsymb53: crate::Reg<huffsymb53::HUFFSYMB53_SPEC>,
    #[doc = "0x2e8 - JPEG HUFFSYMB tables"]
    pub huffsymb54: crate::Reg<huffsymb54::HUFFSYMB54_SPEC>,
    #[doc = "0x2ec - JPEG HUFFSYMB tables"]
    pub huffsymb55: crate::Reg<huffsymb55::HUFFSYMB55_SPEC>,
    #[doc = "0x2f0 - JPEG HUFFSYMB tables"]
    pub huffsymb56: crate::Reg<huffsymb56::HUFFSYMB56_SPEC>,
    #[doc = "0x2f4 - JPEG HUFFSYMB tables"]
    pub huffsymb57: crate::Reg<huffsymb57::HUFFSYMB57_SPEC>,
    #[doc = "0x2f8 - JPEG HUFFSYMB tables"]
    pub huffsymb58: crate::Reg<huffsymb58::HUFFSYMB58_SPEC>,
    #[doc = "0x2fc - JPEG HUFFSYMB tables"]
    pub huffsymb59: crate::Reg<huffsymb59::HUFFSYMB59_SPEC>,
    #[doc = "0x300 - JPEG HUFFSYMB tables"]
    pub huffsymb60: crate::Reg<huffsymb60::HUFFSYMB60_SPEC>,
    #[doc = "0x304 - JPEG HUFFSYMB tables"]
    pub huffsymb61: crate::Reg<huffsymb61::HUFFSYMB61_SPEC>,
    #[doc = "0x308 - JPEG HUFFSYMB tables"]
    pub huffsymb62: crate::Reg<huffsymb62::HUFFSYMB62_SPEC>,
    #[doc = "0x30c - JPEG HUFFSYMB tables"]
    pub huffsymb63: crate::Reg<huffsymb63::HUFFSYMB63_SPEC>,
    #[doc = "0x310 - JPEG HUFFSYMB tables"]
    pub huffsymb64: crate::Reg<huffsymb64::HUFFSYMB64_SPEC>,
    #[doc = "0x314 - JPEG HUFFSYMB tables"]
    pub huffsymb65: crate::Reg<huffsymb65::HUFFSYMB65_SPEC>,
    #[doc = "0x318 - JPEG HUFFSYMB tables"]
    pub huffsymb66: crate::Reg<huffsymb66::HUFFSYMB66_SPEC>,
    #[doc = "0x31c - JPEG HUFFSYMB tables"]
    pub huffsymb67: crate::Reg<huffsymb67::HUFFSYMB67_SPEC>,
    #[doc = "0x320 - JPEG HUFFSYMB tables"]
    pub huffsymb68: crate::Reg<huffsymb68::HUFFSYMB68_SPEC>,
    #[doc = "0x324 - JPEG HUFFSYMB tables"]
    pub huffsymb69: crate::Reg<huffsymb69::HUFFSYMB69_SPEC>,
    #[doc = "0x328 - JPEG HUFFSYMB tables"]
    pub huffsymb70: crate::Reg<huffsymb70::HUFFSYMB70_SPEC>,
    #[doc = "0x32c - JPEG HUFFSYMB tables"]
    pub huffsymb71: crate::Reg<huffsymb71::HUFFSYMB71_SPEC>,
    #[doc = "0x330 - JPEG HUFFSYMB tables"]
    pub huffsymb72: crate::Reg<huffsymb72::HUFFSYMB72_SPEC>,
    #[doc = "0x334 - JPEG HUFFSYMB tables"]
    pub huffsymb73: crate::Reg<huffsymb73::HUFFSYMB73_SPEC>,
    #[doc = "0x338 - JPEG HUFFSYMB tables"]
    pub huffsymb74: crate::Reg<huffsymb74::HUFFSYMB74_SPEC>,
    #[doc = "0x33c - JPEG HUFFSYMB tables"]
    pub huffsymb75: crate::Reg<huffsymb75::HUFFSYMB75_SPEC>,
    #[doc = "0x340 - JPEG HUFFSYMB tables"]
    pub huffsymb76: crate::Reg<huffsymb76::HUFFSYMB76_SPEC>,
    #[doc = "0x344 - JPEG HUFFSYMB tables"]
    pub huffsymb77: crate::Reg<huffsymb77::HUFFSYMB77_SPEC>,
    #[doc = "0x348 - JPEG HUFFSYMB tables"]
    pub huffsymb78: crate::Reg<huffsymb78::HUFFSYMB78_SPEC>,
    #[doc = "0x34c - JPEG HUFFSYMB tables"]
    pub huffsymb79: crate::Reg<huffsymb79::HUFFSYMB79_SPEC>,
    #[doc = "0x350 - JPEG HUFFSYMB tables"]
    pub huffsymb80: crate::Reg<huffsymb80::HUFFSYMB80_SPEC>,
    #[doc = "0x354 - JPEG HUFFSYMB tables"]
    pub huffsymb81: crate::Reg<huffsymb81::HUFFSYMB81_SPEC>,
    #[doc = "0x358 - JPEG HUFFSYMB tables"]
    pub huffsymb82: crate::Reg<huffsymb82::HUFFSYMB82_SPEC>,
    #[doc = "0x35c - JPEG HUFFSYMB tables"]
    pub huffsymb83: crate::Reg<huffsymb83::HUFFSYMB83_SPEC>,
    #[doc = "0x360 - JPEG DHTMem tables"]
    pub dhtmem0: crate::Reg<dhtmem0::DHTMEM0_SPEC>,
    #[doc = "0x364 - JPEG DHTMem tables"]
    pub dhtmem2: crate::Reg<dhtmem2::DHTMEM2_SPEC>,
    #[doc = "0x368 - JPEG DHTMem tables"]
    pub dhtmem3: crate::Reg<dhtmem3::DHTMEM3_SPEC>,
    #[doc = "0x36c - JPEG DHTMem tables"]
    pub dhtmem4: crate::Reg<dhtmem4::DHTMEM4_SPEC>,
    #[doc = "0x370 - JPEG DHTMem tables"]
    pub dhtmem5: crate::Reg<dhtmem5::DHTMEM5_SPEC>,
    #[doc = "0x374 - JPEG DHTMem tables"]
    pub dhtmem6: crate::Reg<dhtmem6::DHTMEM6_SPEC>,
    #[doc = "0x378 - JPEG DHTMem tables"]
    pub dhtmem7: crate::Reg<dhtmem7::DHTMEM7_SPEC>,
    #[doc = "0x37c - JPEG DHTMem tables"]
    pub dhtmem8: crate::Reg<dhtmem8::DHTMEM8_SPEC>,
    #[doc = "0x380 - JPEG DHTMem tables"]
    pub dhtmem9: crate::Reg<dhtmem9::DHTMEM9_SPEC>,
    #[doc = "0x384 - JPEG DHTMem tables"]
    pub dhtmem10: crate::Reg<dhtmem10::DHTMEM10_SPEC>,
    #[doc = "0x388 - JPEG DHTMem tables"]
    pub dhtmem11: crate::Reg<dhtmem11::DHTMEM11_SPEC>,
    #[doc = "0x38c - JPEG DHTMem tables"]
    pub dhtmem12: crate::Reg<dhtmem12::DHTMEM12_SPEC>,
    #[doc = "0x390 - JPEG DHTMem tables"]
    pub dhtmem13: crate::Reg<dhtmem13::DHTMEM13_SPEC>,
    #[doc = "0x394 - JPEG DHTMem tables"]
    pub dhtmem14: crate::Reg<dhtmem14::DHTMEM14_SPEC>,
    #[doc = "0x398 - JPEG DHTMem tables"]
    pub dhtmem15: crate::Reg<dhtmem15::DHTMEM15_SPEC>,
    #[doc = "0x39c - JPEG DHTMem tables"]
    pub dhtmem16: crate::Reg<dhtmem16::DHTMEM16_SPEC>,
    #[doc = "0x3a0 - JPEG DHTMem tables"]
    pub dhtmem17: crate::Reg<dhtmem17::DHTMEM17_SPEC>,
    #[doc = "0x3a4 - JPEG DHTMem tables"]
    pub dhtmem18: crate::Reg<dhtmem18::DHTMEM18_SPEC>,
    #[doc = "0x3a8 - JPEG DHTMem tables"]
    pub dhtmem19: crate::Reg<dhtmem19::DHTMEM19_SPEC>,
    #[doc = "0x3ac - JPEG DHTMem tables"]
    pub dhtmem20: crate::Reg<dhtmem20::DHTMEM20_SPEC>,
    #[doc = "0x3b0 - JPEG DHTMem tables"]
    pub dhtmem21: crate::Reg<dhtmem21::DHTMEM21_SPEC>,
    #[doc = "0x3b4 - JPEG DHTMem tables"]
    pub dhtmem22: crate::Reg<dhtmem22::DHTMEM22_SPEC>,
    #[doc = "0x3b8 - JPEG DHTMem tables"]
    pub dhtmem23: crate::Reg<dhtmem23::DHTMEM23_SPEC>,
    #[doc = "0x3bc - JPEG DHTMem tables"]
    pub dhtmem24: crate::Reg<dhtmem24::DHTMEM24_SPEC>,
    #[doc = "0x3c0 - JPEG DHTMem tables"]
    pub dhtmem25: crate::Reg<dhtmem25::DHTMEM25_SPEC>,
    #[doc = "0x3c4 - JPEG DHTMem tables"]
    pub dhtmem26: crate::Reg<dhtmem26::DHTMEM26_SPEC>,
    #[doc = "0x3c8 - JPEG DHTMem tables"]
    pub dhtmem27: crate::Reg<dhtmem27::DHTMEM27_SPEC>,
    #[doc = "0x3cc - JPEG DHTMem tables"]
    pub dhtmem28: crate::Reg<dhtmem28::DHTMEM28_SPEC>,
    #[doc = "0x3d0 - JPEG DHTMem tables"]
    pub dhtmem29: crate::Reg<dhtmem29::DHTMEM29_SPEC>,
    #[doc = "0x3d4 - JPEG DHTMem tables"]
    pub dhtmem30: crate::Reg<dhtmem30::DHTMEM30_SPEC>,
    #[doc = "0x3d8 - JPEG DHTMem tables"]
    pub dhtmem31: crate::Reg<dhtmem31::DHTMEM31_SPEC>,
    #[doc = "0x3dc - JPEG DHTMem tables"]
    pub dhtmem32: crate::Reg<dhtmem32::DHTMEM32_SPEC>,
    #[doc = "0x3e0 - JPEG DHTMem tables"]
    pub dhtmem33: crate::Reg<dhtmem33::DHTMEM33_SPEC>,
    #[doc = "0x3e4 - JPEG DHTMem tables"]
    pub dhtmem34: crate::Reg<dhtmem34::DHTMEM34_SPEC>,
    #[doc = "0x3e8 - JPEG DHTMem tables"]
    pub dhtmem35: crate::Reg<dhtmem35::DHTMEM35_SPEC>,
    #[doc = "0x3ec - JPEG DHTMem tables"]
    pub dhtmem36: crate::Reg<dhtmem36::DHTMEM36_SPEC>,
    #[doc = "0x3f0 - JPEG DHTMem tables"]
    pub dhtmem37: crate::Reg<dhtmem37::DHTMEM37_SPEC>,
    #[doc = "0x3f4 - JPEG DHTMem tables"]
    pub dhtmem38: crate::Reg<dhtmem38::DHTMEM38_SPEC>,
    #[doc = "0x3f8 - JPEG DHTMem tables"]
    pub dhtmem39: crate::Reg<dhtmem39::DHTMEM39_SPEC>,
    #[doc = "0x3fc - JPEG DHTMem tables"]
    pub dhtmem40: crate::Reg<dhtmem40::DHTMEM40_SPEC>,
    #[doc = "0x400 - JPEG DHTMem tables"]
    pub dhtmem41: crate::Reg<dhtmem41::DHTMEM41_SPEC>,
    #[doc = "0x404 - JPEG DHTMem tables"]
    pub dhtmem42: crate::Reg<dhtmem42::DHTMEM42_SPEC>,
    #[doc = "0x408 - JPEG DHTMem tables"]
    pub dhtmem43: crate::Reg<dhtmem43::DHTMEM43_SPEC>,
    #[doc = "0x40c - JPEG DHTMem tables"]
    pub dhtmem44: crate::Reg<dhtmem44::DHTMEM44_SPEC>,
    #[doc = "0x410 - JPEG DHTMem tables"]
    pub dhtmem45: crate::Reg<dhtmem45::DHTMEM45_SPEC>,
    #[doc = "0x414 - JPEG DHTMem tables"]
    pub dhtmem46: crate::Reg<dhtmem46::DHTMEM46_SPEC>,
    #[doc = "0x418 - JPEG DHTMem tables"]
    pub dhtmem47: crate::Reg<dhtmem47::DHTMEM47_SPEC>,
    #[doc = "0x41c - JPEG DHTMem tables"]
    pub dhtmem48: crate::Reg<dhtmem48::DHTMEM48_SPEC>,
    #[doc = "0x420 - JPEG DHTMem tables"]
    pub dhtmem49: crate::Reg<dhtmem49::DHTMEM49_SPEC>,
    #[doc = "0x424 - JPEG DHTMem tables"]
    pub dhtmem50: crate::Reg<dhtmem50::DHTMEM50_SPEC>,
    #[doc = "0x428 - JPEG DHTMem tables"]
    pub dhtmem51: crate::Reg<dhtmem51::DHTMEM51_SPEC>,
    #[doc = "0x42c - JPEG DHTMem tables"]
    pub dhtmem52: crate::Reg<dhtmem52::DHTMEM52_SPEC>,
    #[doc = "0x430 - JPEG DHTMem tables"]
    pub dhtmem53: crate::Reg<dhtmem53::DHTMEM53_SPEC>,
    #[doc = "0x434 - JPEG DHTMem tables"]
    pub dhtmem54: crate::Reg<dhtmem54::DHTMEM54_SPEC>,
    #[doc = "0x438 - JPEG DHTMem tables"]
    pub dhtmem55: crate::Reg<dhtmem55::DHTMEM55_SPEC>,
    #[doc = "0x43c - JPEG DHTMem tables"]
    pub dhtmem56: crate::Reg<dhtmem56::DHTMEM56_SPEC>,
    #[doc = "0x440 - JPEG DHTMem tables"]
    pub dhtmem57: crate::Reg<dhtmem57::DHTMEM57_SPEC>,
    #[doc = "0x444 - JPEG DHTMem tables"]
    pub dhtmem58: crate::Reg<dhtmem58::DHTMEM58_SPEC>,
    #[doc = "0x448 - JPEG DHTMem tables"]
    pub dhtmem59: crate::Reg<dhtmem59::DHTMEM59_SPEC>,
    #[doc = "0x44c - JPEG DHTMem tables"]
    pub dhtmem60: crate::Reg<dhtmem60::DHTMEM60_SPEC>,
    #[doc = "0x450 - JPEG DHTMem tables"]
    pub dhtmem61: crate::Reg<dhtmem61::DHTMEM61_SPEC>,
    #[doc = "0x454 - JPEG DHTMem tables"]
    pub dhtmem62: crate::Reg<dhtmem62::DHTMEM62_SPEC>,
    #[doc = "0x458 - JPEG DHTMem tables"]
    pub dhtmem63: crate::Reg<dhtmem63::DHTMEM63_SPEC>,
    #[doc = "0x45c - JPEG DHTMem tables"]
    pub dhtmem64: crate::Reg<dhtmem64::DHTMEM64_SPEC>,
    #[doc = "0x460 - JPEG DHTMem tables"]
    pub dhtmem65: crate::Reg<dhtmem65::DHTMEM65_SPEC>,
    #[doc = "0x464 - JPEG DHTMem tables"]
    pub dhtmem66: crate::Reg<dhtmem66::DHTMEM66_SPEC>,
    #[doc = "0x468 - JPEG DHTMem tables"]
    pub dhtmem67: crate::Reg<dhtmem67::DHTMEM67_SPEC>,
    #[doc = "0x46c - JPEG DHTMem tables"]
    pub dhtmem68: crate::Reg<dhtmem68::DHTMEM68_SPEC>,
    #[doc = "0x470 - JPEG DHTMem tables"]
    pub dhtmem69: crate::Reg<dhtmem69::DHTMEM69_SPEC>,
    #[doc = "0x474 - JPEG DHTMem tables"]
    pub dhtmem70: crate::Reg<dhtmem70::DHTMEM70_SPEC>,
    #[doc = "0x478 - JPEG DHTMem tables"]
    pub dhtmem71: crate::Reg<dhtmem71::DHTMEM71_SPEC>,
    #[doc = "0x47c - JPEG DHTMem tables"]
    pub dhtmem72: crate::Reg<dhtmem72::DHTMEM72_SPEC>,
    #[doc = "0x480 - JPEG DHTMem tables"]
    pub dhtmem73: crate::Reg<dhtmem73::DHTMEM73_SPEC>,
    #[doc = "0x484 - JPEG DHTMem tables"]
    pub dhtmem74: crate::Reg<dhtmem74::DHTMEM74_SPEC>,
    #[doc = "0x488 - JPEG DHTMem tables"]
    pub dhtmem75: crate::Reg<dhtmem75::DHTMEM75_SPEC>,
    #[doc = "0x48c - JPEG DHTMem tables"]
    pub dhtmem76: crate::Reg<dhtmem76::DHTMEM76_SPEC>,
    #[doc = "0x490 - JPEG DHTMem tables"]
    pub dhtmem77: crate::Reg<dhtmem77::DHTMEM77_SPEC>,
    #[doc = "0x494 - JPEG DHTMem tables"]
    pub dhtmem78: crate::Reg<dhtmem78::DHTMEM78_SPEC>,
    #[doc = "0x498 - JPEG DHTMem tables"]
    pub dhtmem79: crate::Reg<dhtmem79::DHTMEM79_SPEC>,
    #[doc = "0x49c - JPEG DHTMem tables"]
    pub dhtmem80: crate::Reg<dhtmem80::DHTMEM80_SPEC>,
    #[doc = "0x4a0 - JPEG DHTMem tables"]
    pub dhtmem81: crate::Reg<dhtmem81::DHTMEM81_SPEC>,
    #[doc = "0x4a4 - JPEG DHTMem tables"]
    pub dhtmem82: crate::Reg<dhtmem82::DHTMEM82_SPEC>,
    #[doc = "0x4a8 - JPEG DHTMem tables"]
    pub dhtmem83: crate::Reg<dhtmem83::DHTMEM83_SPEC>,
    #[doc = "0x4ac - JPEG DHTMem tables"]
    pub dhtmem84: crate::Reg<dhtmem84::DHTMEM84_SPEC>,
    #[doc = "0x4b0 - JPEG DHTMem tables"]
    pub dhtmem85: crate::Reg<dhtmem85::DHTMEM85_SPEC>,
    #[doc = "0x4b4 - JPEG DHTMem tables"]
    pub dhtmem86: crate::Reg<dhtmem86::DHTMEM86_SPEC>,
    #[doc = "0x4b8 - JPEG DHTMem tables"]
    pub dhtmem87: crate::Reg<dhtmem87::DHTMEM87_SPEC>,
    #[doc = "0x4bc - JPEG DHTMem tables"]
    pub dhtmem88: crate::Reg<dhtmem88::DHTMEM88_SPEC>,
    #[doc = "0x4c0 - JPEG DHTMem tables"]
    pub dhtmem89: crate::Reg<dhtmem89::DHTMEM89_SPEC>,
    #[doc = "0x4c4 - JPEG DHTMem tables"]
    pub dhtmem90: crate::Reg<dhtmem90::DHTMEM90_SPEC>,
    #[doc = "0x4c8 - JPEG DHTMem tables"]
    pub dhtmem91: crate::Reg<dhtmem91::DHTMEM91_SPEC>,
    #[doc = "0x4cc - JPEG DHTMem tables"]
    pub dhtmem92: crate::Reg<dhtmem92::DHTMEM92_SPEC>,
    #[doc = "0x4d0 - JPEG DHTMem tables"]
    pub dhtmem93: crate::Reg<dhtmem93::DHTMEM93_SPEC>,
    #[doc = "0x4d4 - JPEG DHTMem tables"]
    pub dhtmem94: crate::Reg<dhtmem94::DHTMEM94_SPEC>,
    #[doc = "0x4d8 - JPEG DHTMem tables"]
    pub dhtmem95: crate::Reg<dhtmem95::DHTMEM95_SPEC>,
    #[doc = "0x4dc - JPEG DHTMem tables"]
    pub dhtmem96: crate::Reg<dhtmem96::DHTMEM96_SPEC>,
    #[doc = "0x4e0 - JPEG DHTMem tables"]
    pub dhtmem97: crate::Reg<dhtmem97::DHTMEM97_SPEC>,
    #[doc = "0x4e4 - JPEG DHTMem tables"]
    pub dhtmem98: crate::Reg<dhtmem98::DHTMEM98_SPEC>,
    #[doc = "0x4e8 - JPEG DHTMem tables"]
    pub dhtmem99: crate::Reg<dhtmem99::DHTMEM99_SPEC>,
    #[doc = "0x4ec - JPEG DHTMem tables"]
    pub dhtmem100: crate::Reg<dhtmem100::DHTMEM100_SPEC>,
    #[doc = "0x4f0 - JPEG DHTMem tables"]
    pub dhtmem101: crate::Reg<dhtmem101::DHTMEM101_SPEC>,
    #[doc = "0x4f4 - JPEG DHTMem tables"]
    pub dhtmem102: crate::Reg<dhtmem102::DHTMEM102_SPEC>,
    #[doc = "0x4f8 - JPEG DHTMem tables"]
    pub dhtmem103: crate::Reg<dhtmem103::DHTMEM103_SPEC>,
    _reserved312: [u8; 0x04],
    #[doc = "0x500 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_0: crate::Reg<huffenc_ac0_0::HUFFENC_AC0_0_SPEC>,
    #[doc = "0x504 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_1: crate::Reg<huffenc_ac0_1::HUFFENC_AC0_1_SPEC>,
    #[doc = "0x508 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_2: crate::Reg<huffenc_ac0_2::HUFFENC_AC0_2_SPEC>,
    #[doc = "0x50c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_3: crate::Reg<huffenc_ac0_3::HUFFENC_AC0_3_SPEC>,
    #[doc = "0x510 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_4: crate::Reg<huffenc_ac0_4::HUFFENC_AC0_4_SPEC>,
    #[doc = "0x514 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_5: crate::Reg<huffenc_ac0_5::HUFFENC_AC0_5_SPEC>,
    #[doc = "0x518 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_6: crate::Reg<huffenc_ac0_6::HUFFENC_AC0_6_SPEC>,
    #[doc = "0x51c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_7: crate::Reg<huffenc_ac0_7::HUFFENC_AC0_7_SPEC>,
    #[doc = "0x520 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_8: crate::Reg<huffenc_ac0_8::HUFFENC_AC0_8_SPEC>,
    #[doc = "0x524 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_9: crate::Reg<huffenc_ac0_9::HUFFENC_AC0_9_SPEC>,
    #[doc = "0x528 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_10: crate::Reg<huffenc_ac0_10::HUFFENC_AC0_10_SPEC>,
    #[doc = "0x52c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_11: crate::Reg<huffenc_ac0_11::HUFFENC_AC0_11_SPEC>,
    #[doc = "0x530 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_12: crate::Reg<huffenc_ac0_12::HUFFENC_AC0_12_SPEC>,
    #[doc = "0x534 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_13: crate::Reg<huffenc_ac0_13::HUFFENC_AC0_13_SPEC>,
    #[doc = "0x538 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_14: crate::Reg<huffenc_ac0_14::HUFFENC_AC0_14_SPEC>,
    #[doc = "0x53c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_15: crate::Reg<huffenc_ac0_15::HUFFENC_AC0_15_SPEC>,
    #[doc = "0x540 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_16: crate::Reg<huffenc_ac0_16::HUFFENC_AC0_16_SPEC>,
    #[doc = "0x544 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_17: crate::Reg<huffenc_ac0_17::HUFFENC_AC0_17_SPEC>,
    #[doc = "0x548 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_18: crate::Reg<huffenc_ac0_18::HUFFENC_AC0_18_SPEC>,
    #[doc = "0x54c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_19: crate::Reg<huffenc_ac0_19::HUFFENC_AC0_19_SPEC>,
    #[doc = "0x550 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_20: crate::Reg<huffenc_ac0_20::HUFFENC_AC0_20_SPEC>,
    #[doc = "0x554 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_21: crate::Reg<huffenc_ac0_21::HUFFENC_AC0_21_SPEC>,
    #[doc = "0x558 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_22: crate::Reg<huffenc_ac0_22::HUFFENC_AC0_22_SPEC>,
    #[doc = "0x55c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_23: crate::Reg<huffenc_ac0_23::HUFFENC_AC0_23_SPEC>,
    #[doc = "0x560 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_24: crate::Reg<huffenc_ac0_24::HUFFENC_AC0_24_SPEC>,
    #[doc = "0x564 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_25: crate::Reg<huffenc_ac0_25::HUFFENC_AC0_25_SPEC>,
    #[doc = "0x568 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_26: crate::Reg<huffenc_ac0_26::HUFFENC_AC0_26_SPEC>,
    #[doc = "0x56c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_27: crate::Reg<huffenc_ac0_27::HUFFENC_AC0_27_SPEC>,
    #[doc = "0x570 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_28: crate::Reg<huffenc_ac0_28::HUFFENC_AC0_28_SPEC>,
    #[doc = "0x574 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_29: crate::Reg<huffenc_ac0_29::HUFFENC_AC0_29_SPEC>,
    #[doc = "0x578 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_30: crate::Reg<huffenc_ac0_30::HUFFENC_AC0_30_SPEC>,
    #[doc = "0x57c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_31: crate::Reg<huffenc_ac0_31::HUFFENC_AC0_31_SPEC>,
    #[doc = "0x580 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_32: crate::Reg<huffenc_ac0_32::HUFFENC_AC0_32_SPEC>,
    #[doc = "0x584 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_33: crate::Reg<huffenc_ac0_33::HUFFENC_AC0_33_SPEC>,
    #[doc = "0x588 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_34: crate::Reg<huffenc_ac0_34::HUFFENC_AC0_34_SPEC>,
    #[doc = "0x58c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_35: crate::Reg<huffenc_ac0_35::HUFFENC_AC0_35_SPEC>,
    #[doc = "0x590 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_36: crate::Reg<huffenc_ac0_36::HUFFENC_AC0_36_SPEC>,
    #[doc = "0x594 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_37: crate::Reg<huffenc_ac0_37::HUFFENC_AC0_37_SPEC>,
    #[doc = "0x598 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_38: crate::Reg<huffenc_ac0_38::HUFFENC_AC0_38_SPEC>,
    #[doc = "0x59c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_39: crate::Reg<huffenc_ac0_39::HUFFENC_AC0_39_SPEC>,
    #[doc = "0x5a0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_40: crate::Reg<huffenc_ac0_40::HUFFENC_AC0_40_SPEC>,
    #[doc = "0x5a4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_41: crate::Reg<huffenc_ac0_41::HUFFENC_AC0_41_SPEC>,
    #[doc = "0x5a8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_42: crate::Reg<huffenc_ac0_42::HUFFENC_AC0_42_SPEC>,
    #[doc = "0x5ac - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_43: crate::Reg<huffenc_ac0_43::HUFFENC_AC0_43_SPEC>,
    #[doc = "0x5b0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_44: crate::Reg<huffenc_ac0_44::HUFFENC_AC0_44_SPEC>,
    #[doc = "0x5b4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_45: crate::Reg<huffenc_ac0_45::HUFFENC_AC0_45_SPEC>,
    #[doc = "0x5b8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_46: crate::Reg<huffenc_ac0_46::HUFFENC_AC0_46_SPEC>,
    #[doc = "0x5bc - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_47: crate::Reg<huffenc_ac0_47::HUFFENC_AC0_47_SPEC>,
    #[doc = "0x5c0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_48: crate::Reg<huffenc_ac0_48::HUFFENC_AC0_48_SPEC>,
    #[doc = "0x5c4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_49: crate::Reg<huffenc_ac0_49::HUFFENC_AC0_49_SPEC>,
    #[doc = "0x5c8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_50: crate::Reg<huffenc_ac0_50::HUFFENC_AC0_50_SPEC>,
    #[doc = "0x5cc - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_51: crate::Reg<huffenc_ac0_51::HUFFENC_AC0_51_SPEC>,
    #[doc = "0x5d0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_52: crate::Reg<huffenc_ac0_52::HUFFENC_AC0_52_SPEC>,
    #[doc = "0x5d4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_53: crate::Reg<huffenc_ac0_53::HUFFENC_AC0_53_SPEC>,
    #[doc = "0x5d8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_54: crate::Reg<huffenc_ac0_54::HUFFENC_AC0_54_SPEC>,
    #[doc = "0x5dc - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_55: crate::Reg<huffenc_ac0_55::HUFFENC_AC0_55_SPEC>,
    #[doc = "0x5e0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_56: crate::Reg<huffenc_ac0_56::HUFFENC_AC0_56_SPEC>,
    #[doc = "0x5e4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_57: crate::Reg<huffenc_ac0_57::HUFFENC_AC0_57_SPEC>,
    #[doc = "0x5e8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_58: crate::Reg<huffenc_ac0_58::HUFFENC_AC0_58_SPEC>,
    #[doc = "0x5ec - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_59: crate::Reg<huffenc_ac0_59::HUFFENC_AC0_59_SPEC>,
    #[doc = "0x5f0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_60: crate::Reg<huffenc_ac0_60::HUFFENC_AC0_60_SPEC>,
    #[doc = "0x5f4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_61: crate::Reg<huffenc_ac0_61::HUFFENC_AC0_61_SPEC>,
    #[doc = "0x5f8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_62: crate::Reg<huffenc_ac0_62::HUFFENC_AC0_62_SPEC>,
    #[doc = "0x5fc - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_63: crate::Reg<huffenc_ac0_63::HUFFENC_AC0_63_SPEC>,
    #[doc = "0x600 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_64: crate::Reg<huffenc_ac0_64::HUFFENC_AC0_64_SPEC>,
    #[doc = "0x604 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_65: crate::Reg<huffenc_ac0_65::HUFFENC_AC0_65_SPEC>,
    #[doc = "0x608 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_66: crate::Reg<huffenc_ac0_66::HUFFENC_AC0_66_SPEC>,
    #[doc = "0x60c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_67: crate::Reg<huffenc_ac0_67::HUFFENC_AC0_67_SPEC>,
    #[doc = "0x610 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_68: crate::Reg<huffenc_ac0_68::HUFFENC_AC0_68_SPEC>,
    #[doc = "0x614 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_69: crate::Reg<huffenc_ac0_69::HUFFENC_AC0_69_SPEC>,
    #[doc = "0x618 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_70: crate::Reg<huffenc_ac0_70::HUFFENC_AC0_70_SPEC>,
    #[doc = "0x61c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_71: crate::Reg<huffenc_ac0_71::HUFFENC_AC0_71_SPEC>,
    #[doc = "0x620 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_72: crate::Reg<huffenc_ac0_72::HUFFENC_AC0_72_SPEC>,
    #[doc = "0x624 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_73: crate::Reg<huffenc_ac0_73::HUFFENC_AC0_73_SPEC>,
    #[doc = "0x628 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_74: crate::Reg<huffenc_ac0_74::HUFFENC_AC0_74_SPEC>,
    #[doc = "0x62c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_75: crate::Reg<huffenc_ac0_75::HUFFENC_AC0_75_SPEC>,
    #[doc = "0x630 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_76: crate::Reg<huffenc_ac0_76::HUFFENC_AC0_76_SPEC>,
    #[doc = "0x634 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_77: crate::Reg<huffenc_ac0_77::HUFFENC_AC0_77_SPEC>,
    #[doc = "0x638 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_78: crate::Reg<huffenc_ac0_78::HUFFENC_AC0_78_SPEC>,
    #[doc = "0x63c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_79: crate::Reg<huffenc_ac0_79::HUFFENC_AC0_79_SPEC>,
    #[doc = "0x640 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_80: crate::Reg<huffenc_ac0_80::HUFFENC_AC0_80_SPEC>,
    #[doc = "0x644 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_81: crate::Reg<huffenc_ac0_81::HUFFENC_AC0_81_SPEC>,
    #[doc = "0x648 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_82: crate::Reg<huffenc_ac0_82::HUFFENC_AC0_82_SPEC>,
    #[doc = "0x64c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_83: crate::Reg<huffenc_ac0_83::HUFFENC_AC0_83_SPEC>,
    #[doc = "0x650 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_84: crate::Reg<huffenc_ac0_84::HUFFENC_AC0_84_SPEC>,
    #[doc = "0x654 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_85: crate::Reg<huffenc_ac0_85::HUFFENC_AC0_85_SPEC>,
    #[doc = "0x658 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_86: crate::Reg<huffenc_ac0_86::HUFFENC_AC0_86_SPEC>,
    #[doc = "0x65c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_87: crate::Reg<huffenc_ac0_87::HUFFENC_AC0_87_SPEC>,
    #[doc = "0x660 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_0: crate::Reg<huffenc_ac1_0::HUFFENC_AC1_0_SPEC>,
    #[doc = "0x664 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_1: crate::Reg<huffenc_ac1_1::HUFFENC_AC1_1_SPEC>,
    #[doc = "0x668 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_2: crate::Reg<huffenc_ac1_2::HUFFENC_AC1_2_SPEC>,
    #[doc = "0x66c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_3: crate::Reg<huffenc_ac1_3::HUFFENC_AC1_3_SPEC>,
    #[doc = "0x670 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_4: crate::Reg<huffenc_ac1_4::HUFFENC_AC1_4_SPEC>,
    #[doc = "0x674 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_5: crate::Reg<huffenc_ac1_5::HUFFENC_AC1_5_SPEC>,
    #[doc = "0x678 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_6: crate::Reg<huffenc_ac1_6::HUFFENC_AC1_6_SPEC>,
    #[doc = "0x67c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_7: crate::Reg<huffenc_ac1_7::HUFFENC_AC1_7_SPEC>,
    #[doc = "0x680 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_8: crate::Reg<huffenc_ac1_8::HUFFENC_AC1_8_SPEC>,
    #[doc = "0x684 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_9: crate::Reg<huffenc_ac1_9::HUFFENC_AC1_9_SPEC>,
    #[doc = "0x688 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_10: crate::Reg<huffenc_ac1_10::HUFFENC_AC1_10_SPEC>,
    #[doc = "0x68c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_11: crate::Reg<huffenc_ac1_11::HUFFENC_AC1_11_SPEC>,
    #[doc = "0x690 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_12: crate::Reg<huffenc_ac1_12::HUFFENC_AC1_12_SPEC>,
    #[doc = "0x694 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_13: crate::Reg<huffenc_ac1_13::HUFFENC_AC1_13_SPEC>,
    #[doc = "0x698 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_14: crate::Reg<huffenc_ac1_14::HUFFENC_AC1_14_SPEC>,
    #[doc = "0x69c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_15: crate::Reg<huffenc_ac1_15::HUFFENC_AC1_15_SPEC>,
    #[doc = "0x6a0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_16: crate::Reg<huffenc_ac1_16::HUFFENC_AC1_16_SPEC>,
    #[doc = "0x6a4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_17: crate::Reg<huffenc_ac1_17::HUFFENC_AC1_17_SPEC>,
    #[doc = "0x6a8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_18: crate::Reg<huffenc_ac1_18::HUFFENC_AC1_18_SPEC>,
    #[doc = "0x6ac - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_19: crate::Reg<huffenc_ac1_19::HUFFENC_AC1_19_SPEC>,
    #[doc = "0x6b0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_20: crate::Reg<huffenc_ac1_20::HUFFENC_AC1_20_SPEC>,
    #[doc = "0x6b4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_21: crate::Reg<huffenc_ac1_21::HUFFENC_AC1_21_SPEC>,
    #[doc = "0x6b8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_22: crate::Reg<huffenc_ac1_22::HUFFENC_AC1_22_SPEC>,
    #[doc = "0x6bc - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_23: crate::Reg<huffenc_ac1_23::HUFFENC_AC1_23_SPEC>,
    #[doc = "0x6c0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_24: crate::Reg<huffenc_ac1_24::HUFFENC_AC1_24_SPEC>,
    #[doc = "0x6c4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_25: crate::Reg<huffenc_ac1_25::HUFFENC_AC1_25_SPEC>,
    #[doc = "0x6c8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_26: crate::Reg<huffenc_ac1_26::HUFFENC_AC1_26_SPEC>,
    #[doc = "0x6cc - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_27: crate::Reg<huffenc_ac1_27::HUFFENC_AC1_27_SPEC>,
    #[doc = "0x6d0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_28: crate::Reg<huffenc_ac1_28::HUFFENC_AC1_28_SPEC>,
    #[doc = "0x6d4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_29: crate::Reg<huffenc_ac1_29::HUFFENC_AC1_29_SPEC>,
    #[doc = "0x6d8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_30: crate::Reg<huffenc_ac1_30::HUFFENC_AC1_30_SPEC>,
    #[doc = "0x6dc - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_31: crate::Reg<huffenc_ac1_31::HUFFENC_AC1_31_SPEC>,
    #[doc = "0x6e0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_32: crate::Reg<huffenc_ac1_32::HUFFENC_AC1_32_SPEC>,
    #[doc = "0x6e4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_33: crate::Reg<huffenc_ac1_33::HUFFENC_AC1_33_SPEC>,
    #[doc = "0x6e8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_34: crate::Reg<huffenc_ac1_34::HUFFENC_AC1_34_SPEC>,
    #[doc = "0x6ec - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_35: crate::Reg<huffenc_ac1_35::HUFFENC_AC1_35_SPEC>,
    #[doc = "0x6f0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_36: crate::Reg<huffenc_ac1_36::HUFFENC_AC1_36_SPEC>,
    #[doc = "0x6f4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_37: crate::Reg<huffenc_ac1_37::HUFFENC_AC1_37_SPEC>,
    #[doc = "0x6f8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_38: crate::Reg<huffenc_ac1_38::HUFFENC_AC1_38_SPEC>,
    #[doc = "0x6fc - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_39: crate::Reg<huffenc_ac1_39::HUFFENC_AC1_39_SPEC>,
    #[doc = "0x700 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_40: crate::Reg<huffenc_ac1_40::HUFFENC_AC1_40_SPEC>,
    #[doc = "0x704 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_41: crate::Reg<huffenc_ac1_41::HUFFENC_AC1_41_SPEC>,
    #[doc = "0x708 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_42: crate::Reg<huffenc_ac1_42::HUFFENC_AC1_42_SPEC>,
    #[doc = "0x70c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_43: crate::Reg<huffenc_ac1_43::HUFFENC_AC1_43_SPEC>,
    #[doc = "0x710 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_44: crate::Reg<huffenc_ac1_44::HUFFENC_AC1_44_SPEC>,
    #[doc = "0x714 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_45: crate::Reg<huffenc_ac1_45::HUFFENC_AC1_45_SPEC>,
    #[doc = "0x718 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_46: crate::Reg<huffenc_ac1_46::HUFFENC_AC1_46_SPEC>,
    #[doc = "0x71c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_47: crate::Reg<huffenc_ac1_47::HUFFENC_AC1_47_SPEC>,
    #[doc = "0x720 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_48: crate::Reg<huffenc_ac1_48::HUFFENC_AC1_48_SPEC>,
    #[doc = "0x724 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_49: crate::Reg<huffenc_ac1_49::HUFFENC_AC1_49_SPEC>,
    #[doc = "0x728 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_50: crate::Reg<huffenc_ac1_50::HUFFENC_AC1_50_SPEC>,
    #[doc = "0x72c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_51: crate::Reg<huffenc_ac1_51::HUFFENC_AC1_51_SPEC>,
    #[doc = "0x730 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_52: crate::Reg<huffenc_ac1_52::HUFFENC_AC1_52_SPEC>,
    #[doc = "0x734 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_53: crate::Reg<huffenc_ac1_53::HUFFENC_AC1_53_SPEC>,
    #[doc = "0x738 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_54: crate::Reg<huffenc_ac1_54::HUFFENC_AC1_54_SPEC>,
    #[doc = "0x73c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_55: crate::Reg<huffenc_ac1_55::HUFFENC_AC1_55_SPEC>,
    #[doc = "0x740 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_56: crate::Reg<huffenc_ac1_56::HUFFENC_AC1_56_SPEC>,
    #[doc = "0x744 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_57: crate::Reg<huffenc_ac1_57::HUFFENC_AC1_57_SPEC>,
    #[doc = "0x748 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_58: crate::Reg<huffenc_ac1_58::HUFFENC_AC1_58_SPEC>,
    #[doc = "0x74c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_59: crate::Reg<huffenc_ac1_59::HUFFENC_AC1_59_SPEC>,
    #[doc = "0x750 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_60: crate::Reg<huffenc_ac1_60::HUFFENC_AC1_60_SPEC>,
    #[doc = "0x754 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_61: crate::Reg<huffenc_ac1_61::HUFFENC_AC1_61_SPEC>,
    #[doc = "0x758 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_62: crate::Reg<huffenc_ac1_62::HUFFENC_AC1_62_SPEC>,
    #[doc = "0x75c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_63: crate::Reg<huffenc_ac1_63::HUFFENC_AC1_63_SPEC>,
    #[doc = "0x760 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_64: crate::Reg<huffenc_ac1_64::HUFFENC_AC1_64_SPEC>,
    #[doc = "0x764 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_65: crate::Reg<huffenc_ac1_65::HUFFENC_AC1_65_SPEC>,
    #[doc = "0x768 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_66: crate::Reg<huffenc_ac1_66::HUFFENC_AC1_66_SPEC>,
    #[doc = "0x76c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_67: crate::Reg<huffenc_ac1_67::HUFFENC_AC1_67_SPEC>,
    #[doc = "0x770 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_68: crate::Reg<huffenc_ac1_68::HUFFENC_AC1_68_SPEC>,
    #[doc = "0x774 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_69: crate::Reg<huffenc_ac1_69::HUFFENC_AC1_69_SPEC>,
    #[doc = "0x778 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_70: crate::Reg<huffenc_ac1_70::HUFFENC_AC1_70_SPEC>,
    #[doc = "0x77c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_71: crate::Reg<huffenc_ac1_71::HUFFENC_AC1_71_SPEC>,
    #[doc = "0x780 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_72: crate::Reg<huffenc_ac1_72::HUFFENC_AC1_72_SPEC>,
    #[doc = "0x784 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_73: crate::Reg<huffenc_ac1_73::HUFFENC_AC1_73_SPEC>,
    #[doc = "0x788 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_74: crate::Reg<huffenc_ac1_74::HUFFENC_AC1_74_SPEC>,
    #[doc = "0x78c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_75: crate::Reg<huffenc_ac1_75::HUFFENC_AC1_75_SPEC>,
    #[doc = "0x790 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_76: crate::Reg<huffenc_ac1_76::HUFFENC_AC1_76_SPEC>,
    #[doc = "0x794 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_77: crate::Reg<huffenc_ac1_77::HUFFENC_AC1_77_SPEC>,
    #[doc = "0x798 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_78: crate::Reg<huffenc_ac1_78::HUFFENC_AC1_78_SPEC>,
    #[doc = "0x79c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_79: crate::Reg<huffenc_ac1_79::HUFFENC_AC1_79_SPEC>,
    #[doc = "0x7a0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_80: crate::Reg<huffenc_ac1_80::HUFFENC_AC1_80_SPEC>,
    #[doc = "0x7a4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_81: crate::Reg<huffenc_ac1_81::HUFFENC_AC1_81_SPEC>,
    #[doc = "0x7a8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_82: crate::Reg<huffenc_ac1_82::HUFFENC_AC1_82_SPEC>,
    #[doc = "0x7ac - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_83: crate::Reg<huffenc_ac1_83::HUFFENC_AC1_83_SPEC>,
    #[doc = "0x7b0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_84: crate::Reg<huffenc_ac1_84::HUFFENC_AC1_84_SPEC>,
    #[doc = "0x7b4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_85: crate::Reg<huffenc_ac1_85::HUFFENC_AC1_85_SPEC>,
    #[doc = "0x7b8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_86: crate::Reg<huffenc_ac1_86::HUFFENC_AC1_86_SPEC>,
    #[doc = "0x7bc - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_87: crate::Reg<huffenc_ac1_87::HUFFENC_AC1_87_SPEC>,
    #[doc = "0x7c0 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_0: crate::Reg<huffenc_dc0_0::HUFFENC_DC0_0_SPEC>,
    #[doc = "0x7c4 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_1: crate::Reg<huffenc_dc0_1::HUFFENC_DC0_1_SPEC>,
    #[doc = "0x7c8 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_2: crate::Reg<huffenc_dc0_2::HUFFENC_DC0_2_SPEC>,
    #[doc = "0x7cc - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_3: crate::Reg<huffenc_dc0_3::HUFFENC_DC0_3_SPEC>,
    #[doc = "0x7d0 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_4: crate::Reg<huffenc_dc0_4::HUFFENC_DC0_4_SPEC>,
    #[doc = "0x7d4 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_5: crate::Reg<huffenc_dc0_5::HUFFENC_DC0_5_SPEC>,
    #[doc = "0x7d8 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_6: crate::Reg<huffenc_dc0_6::HUFFENC_DC0_6_SPEC>,
    #[doc = "0x7dc - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_7: crate::Reg<huffenc_dc0_7::HUFFENC_DC0_7_SPEC>,
    #[doc = "0x7e0 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_0: crate::Reg<huffenc_dc1_0::HUFFENC_DC1_0_SPEC>,
    #[doc = "0x7e4 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_1: crate::Reg<huffenc_dc1_1::HUFFENC_DC1_1_SPEC>,
    #[doc = "0x7e8 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_2: crate::Reg<huffenc_dc1_2::HUFFENC_DC1_2_SPEC>,
    #[doc = "0x7ec - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_3: crate::Reg<huffenc_dc1_3::HUFFENC_DC1_3_SPEC>,
    #[doc = "0x7f0 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_4: crate::Reg<huffenc_dc1_4::HUFFENC_DC1_4_SPEC>,
    #[doc = "0x7f4 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_5: crate::Reg<huffenc_dc1_5::HUFFENC_DC1_5_SPEC>,
    #[doc = "0x7f8 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_6: crate::Reg<huffenc_dc1_6::HUFFENC_DC1_6_SPEC>,
    #[doc = "0x7fc - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_7: crate::Reg<huffenc_dc1_7::HUFFENC_DC1_7_SPEC>,
}
#[doc = "JPEG_CONFR0 register accessor: an alias for `Reg<JPEG_CONFR0_SPEC>`"]
pub type JPEG_CONFR0 = crate::Reg<jpeg_confr0::JPEG_CONFR0_SPEC>;
#[doc = "JPEG codec configuration register 0"]
pub mod jpeg_confr0;
#[doc = "JPEG_CONFR1 register accessor: an alias for `Reg<JPEG_CONFR1_SPEC>`"]
pub type JPEG_CONFR1 = crate::Reg<jpeg_confr1::JPEG_CONFR1_SPEC>;
#[doc = "JPEG codec configuration register 1"]
pub mod jpeg_confr1;
#[doc = "JPEG_CONFR2 register accessor: an alias for `Reg<JPEG_CONFR2_SPEC>`"]
pub type JPEG_CONFR2 = crate::Reg<jpeg_confr2::JPEG_CONFR2_SPEC>;
#[doc = "JPEG codec configuration register 2"]
pub mod jpeg_confr2;
#[doc = "JPEG_CONFR3 register accessor: an alias for `Reg<JPEG_CONFR3_SPEC>`"]
pub type JPEG_CONFR3 = crate::Reg<jpeg_confr3::JPEG_CONFR3_SPEC>;
#[doc = "JPEG codec configuration register 3"]
pub mod jpeg_confr3;
#[doc = "JPEG_CONFR4 register accessor: an alias for `Reg<JPEG_CONFR4_SPEC>`"]
pub type JPEG_CONFR4 = crate::Reg<jpeg_confr4::JPEG_CONFR4_SPEC>;
#[doc = "JPEG codec configuration register 4"]
pub mod jpeg_confr4;
#[doc = "JPEG_CONFR5 register accessor: an alias for `Reg<JPEG_CONFR5_SPEC>`"]
pub type JPEG_CONFR5 = crate::Reg<jpeg_confr5::JPEG_CONFR5_SPEC>;
#[doc = "JPEG codec configuration register 5"]
pub mod jpeg_confr5;
#[doc = "JPEG_CONFR6 register accessor: an alias for `Reg<JPEG_CONFR6_SPEC>`"]
pub type JPEG_CONFR6 = crate::Reg<jpeg_confr6::JPEG_CONFR6_SPEC>;
#[doc = "JPEG codec configuration register 6"]
pub mod jpeg_confr6;
#[doc = "JPEG_CONFR7 register accessor: an alias for `Reg<JPEG_CONFR7_SPEC>`"]
pub type JPEG_CONFR7 = crate::Reg<jpeg_confr7::JPEG_CONFR7_SPEC>;
#[doc = "JPEG codec configuration register 7"]
pub mod jpeg_confr7;
#[doc = "JPEG_CR register accessor: an alias for `Reg<JPEG_CR_SPEC>`"]
pub type JPEG_CR = crate::Reg<jpeg_cr::JPEG_CR_SPEC>;
#[doc = "JPEG control register"]
pub mod jpeg_cr;
#[doc = "JPEG_SR register accessor: an alias for `Reg<JPEG_SR_SPEC>`"]
pub type JPEG_SR = crate::Reg<jpeg_sr::JPEG_SR_SPEC>;
#[doc = "JPEG status register"]
pub mod jpeg_sr;
#[doc = "JPEG_CFR register accessor: an alias for `Reg<JPEG_CFR_SPEC>`"]
pub type JPEG_CFR = crate::Reg<jpeg_cfr::JPEG_CFR_SPEC>;
#[doc = "JPEG clear flag register"]
pub mod jpeg_cfr;
#[doc = "JPEG_DIR register accessor: an alias for `Reg<JPEG_DIR_SPEC>`"]
pub type JPEG_DIR = crate::Reg<jpeg_dir::JPEG_DIR_SPEC>;
#[doc = "JPEG data input register"]
pub mod jpeg_dir;
#[doc = "JPEG_DOR register accessor: an alias for `Reg<JPEG_DOR_SPEC>`"]
pub type JPEG_DOR = crate::Reg<jpeg_dor::JPEG_DOR_SPEC>;
#[doc = "JPEG data output register"]
pub mod jpeg_dor;
#[doc = "QMEM0_0 register accessor: an alias for `Reg<QMEM0_0_SPEC>`"]
pub type QMEM0_0 = crate::Reg<qmem0_0::QMEM0_0_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_0;
#[doc = "QMEM0_1 register accessor: an alias for `Reg<QMEM0_1_SPEC>`"]
pub type QMEM0_1 = crate::Reg<qmem0_1::QMEM0_1_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_1;
#[doc = "QMEM0_2 register accessor: an alias for `Reg<QMEM0_2_SPEC>`"]
pub type QMEM0_2 = crate::Reg<qmem0_2::QMEM0_2_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_2;
#[doc = "QMEM0_3 register accessor: an alias for `Reg<QMEM0_3_SPEC>`"]
pub type QMEM0_3 = crate::Reg<qmem0_3::QMEM0_3_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_3;
#[doc = "QMEM0_4 register accessor: an alias for `Reg<QMEM0_4_SPEC>`"]
pub type QMEM0_4 = crate::Reg<qmem0_4::QMEM0_4_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_4;
#[doc = "QMEM0_5 register accessor: an alias for `Reg<QMEM0_5_SPEC>`"]
pub type QMEM0_5 = crate::Reg<qmem0_5::QMEM0_5_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_5;
#[doc = "QMEM0_6 register accessor: an alias for `Reg<QMEM0_6_SPEC>`"]
pub type QMEM0_6 = crate::Reg<qmem0_6::QMEM0_6_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_6;
#[doc = "QMEM0_7 register accessor: an alias for `Reg<QMEM0_7_SPEC>`"]
pub type QMEM0_7 = crate::Reg<qmem0_7::QMEM0_7_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_7;
#[doc = "QMEM0_8 register accessor: an alias for `Reg<QMEM0_8_SPEC>`"]
pub type QMEM0_8 = crate::Reg<qmem0_8::QMEM0_8_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_8;
#[doc = "QMEM0_9 register accessor: an alias for `Reg<QMEM0_9_SPEC>`"]
pub type QMEM0_9 = crate::Reg<qmem0_9::QMEM0_9_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_9;
#[doc = "QMEM0_10 register accessor: an alias for `Reg<QMEM0_10_SPEC>`"]
pub type QMEM0_10 = crate::Reg<qmem0_10::QMEM0_10_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_10;
#[doc = "QMEM0_11 register accessor: an alias for `Reg<QMEM0_11_SPEC>`"]
pub type QMEM0_11 = crate::Reg<qmem0_11::QMEM0_11_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_11;
#[doc = "QMEM0_12 register accessor: an alias for `Reg<QMEM0_12_SPEC>`"]
pub type QMEM0_12 = crate::Reg<qmem0_12::QMEM0_12_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_12;
#[doc = "QMEM0_13 register accessor: an alias for `Reg<QMEM0_13_SPEC>`"]
pub type QMEM0_13 = crate::Reg<qmem0_13::QMEM0_13_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_13;
#[doc = "QMEM0_14 register accessor: an alias for `Reg<QMEM0_14_SPEC>`"]
pub type QMEM0_14 = crate::Reg<qmem0_14::QMEM0_14_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_14;
#[doc = "QMEM0_15 register accessor: an alias for `Reg<QMEM0_15_SPEC>`"]
pub type QMEM0_15 = crate::Reg<qmem0_15::QMEM0_15_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0_15;
#[doc = "QMEM1_0 register accessor: an alias for `Reg<QMEM1_0_SPEC>`"]
pub type QMEM1_0 = crate::Reg<qmem1_0::QMEM1_0_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_0;
#[doc = "QMEM1_1 register accessor: an alias for `Reg<QMEM1_1_SPEC>`"]
pub type QMEM1_1 = crate::Reg<qmem1_1::QMEM1_1_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_1;
#[doc = "QMEM1_2 register accessor: an alias for `Reg<QMEM1_2_SPEC>`"]
pub type QMEM1_2 = crate::Reg<qmem1_2::QMEM1_2_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_2;
#[doc = "QMEM1_3 register accessor: an alias for `Reg<QMEM1_3_SPEC>`"]
pub type QMEM1_3 = crate::Reg<qmem1_3::QMEM1_3_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_3;
#[doc = "QMEM1_4 register accessor: an alias for `Reg<QMEM1_4_SPEC>`"]
pub type QMEM1_4 = crate::Reg<qmem1_4::QMEM1_4_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_4;
#[doc = "QMEM1_5 register accessor: an alias for `Reg<QMEM1_5_SPEC>`"]
pub type QMEM1_5 = crate::Reg<qmem1_5::QMEM1_5_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_5;
#[doc = "QMEM1_6 register accessor: an alias for `Reg<QMEM1_6_SPEC>`"]
pub type QMEM1_6 = crate::Reg<qmem1_6::QMEM1_6_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_6;
#[doc = "QMEM1_7 register accessor: an alias for `Reg<QMEM1_7_SPEC>`"]
pub type QMEM1_7 = crate::Reg<qmem1_7::QMEM1_7_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_7;
#[doc = "QMEM1_8 register accessor: an alias for `Reg<QMEM1_8_SPEC>`"]
pub type QMEM1_8 = crate::Reg<qmem1_8::QMEM1_8_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_8;
#[doc = "QMEM1_9 register accessor: an alias for `Reg<QMEM1_9_SPEC>`"]
pub type QMEM1_9 = crate::Reg<qmem1_9::QMEM1_9_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_9;
#[doc = "QMEM1_10 register accessor: an alias for `Reg<QMEM1_10_SPEC>`"]
pub type QMEM1_10 = crate::Reg<qmem1_10::QMEM1_10_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_10;
#[doc = "QMEM1_11 register accessor: an alias for `Reg<QMEM1_11_SPEC>`"]
pub type QMEM1_11 = crate::Reg<qmem1_11::QMEM1_11_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_11;
#[doc = "QMEM1_12 register accessor: an alias for `Reg<QMEM1_12_SPEC>`"]
pub type QMEM1_12 = crate::Reg<qmem1_12::QMEM1_12_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_12;
#[doc = "QMEM1_13 register accessor: an alias for `Reg<QMEM1_13_SPEC>`"]
pub type QMEM1_13 = crate::Reg<qmem1_13::QMEM1_13_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_13;
#[doc = "QMEM1_14 register accessor: an alias for `Reg<QMEM1_14_SPEC>`"]
pub type QMEM1_14 = crate::Reg<qmem1_14::QMEM1_14_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_14;
#[doc = "QMEM1_15 register accessor: an alias for `Reg<QMEM1_15_SPEC>`"]
pub type QMEM1_15 = crate::Reg<qmem1_15::QMEM1_15_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1_15;
#[doc = "QMEM2_0 register accessor: an alias for `Reg<QMEM2_0_SPEC>`"]
pub type QMEM2_0 = crate::Reg<qmem2_0::QMEM2_0_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_0;
#[doc = "QMEM2_1 register accessor: an alias for `Reg<QMEM2_1_SPEC>`"]
pub type QMEM2_1 = crate::Reg<qmem2_1::QMEM2_1_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_1;
#[doc = "QMEM2_2 register accessor: an alias for `Reg<QMEM2_2_SPEC>`"]
pub type QMEM2_2 = crate::Reg<qmem2_2::QMEM2_2_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_2;
#[doc = "QMEM2_3 register accessor: an alias for `Reg<QMEM2_3_SPEC>`"]
pub type QMEM2_3 = crate::Reg<qmem2_3::QMEM2_3_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_3;
#[doc = "QMEM2_4 register accessor: an alias for `Reg<QMEM2_4_SPEC>`"]
pub type QMEM2_4 = crate::Reg<qmem2_4::QMEM2_4_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_4;
#[doc = "QMEM2_5 register accessor: an alias for `Reg<QMEM2_5_SPEC>`"]
pub type QMEM2_5 = crate::Reg<qmem2_5::QMEM2_5_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_5;
#[doc = "QMEM2_6 register accessor: an alias for `Reg<QMEM2_6_SPEC>`"]
pub type QMEM2_6 = crate::Reg<qmem2_6::QMEM2_6_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_6;
#[doc = "QMEM2_7 register accessor: an alias for `Reg<QMEM2_7_SPEC>`"]
pub type QMEM2_7 = crate::Reg<qmem2_7::QMEM2_7_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_7;
#[doc = "QMEM2_8 register accessor: an alias for `Reg<QMEM2_8_SPEC>`"]
pub type QMEM2_8 = crate::Reg<qmem2_8::QMEM2_8_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_8;
#[doc = "QMEM2_9 register accessor: an alias for `Reg<QMEM2_9_SPEC>`"]
pub type QMEM2_9 = crate::Reg<qmem2_9::QMEM2_9_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_9;
#[doc = "QMEM2_10 register accessor: an alias for `Reg<QMEM2_10_SPEC>`"]
pub type QMEM2_10 = crate::Reg<qmem2_10::QMEM2_10_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_10;
#[doc = "QMEM2_11 register accessor: an alias for `Reg<QMEM2_11_SPEC>`"]
pub type QMEM2_11 = crate::Reg<qmem2_11::QMEM2_11_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_11;
#[doc = "QMEM2_12 register accessor: an alias for `Reg<QMEM2_12_SPEC>`"]
pub type QMEM2_12 = crate::Reg<qmem2_12::QMEM2_12_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_12;
#[doc = "QMEM2_13 register accessor: an alias for `Reg<QMEM2_13_SPEC>`"]
pub type QMEM2_13 = crate::Reg<qmem2_13::QMEM2_13_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_13;
#[doc = "QMEM2_14 register accessor: an alias for `Reg<QMEM2_14_SPEC>`"]
pub type QMEM2_14 = crate::Reg<qmem2_14::QMEM2_14_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_14;
#[doc = "QMEM2_15 register accessor: an alias for `Reg<QMEM2_15_SPEC>`"]
pub type QMEM2_15 = crate::Reg<qmem2_15::QMEM2_15_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2_15;
#[doc = "QMEM3_0 register accessor: an alias for `Reg<QMEM3_0_SPEC>`"]
pub type QMEM3_0 = crate::Reg<qmem3_0::QMEM3_0_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_0;
#[doc = "QMEM3_1 register accessor: an alias for `Reg<QMEM3_1_SPEC>`"]
pub type QMEM3_1 = crate::Reg<qmem3_1::QMEM3_1_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_1;
#[doc = "QMEM3_2 register accessor: an alias for `Reg<QMEM3_2_SPEC>`"]
pub type QMEM3_2 = crate::Reg<qmem3_2::QMEM3_2_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_2;
#[doc = "QMEM3_3 register accessor: an alias for `Reg<QMEM3_3_SPEC>`"]
pub type QMEM3_3 = crate::Reg<qmem3_3::QMEM3_3_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_3;
#[doc = "QMEM3_4 register accessor: an alias for `Reg<QMEM3_4_SPEC>`"]
pub type QMEM3_4 = crate::Reg<qmem3_4::QMEM3_4_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_4;
#[doc = "QMEM3_5 register accessor: an alias for `Reg<QMEM3_5_SPEC>`"]
pub type QMEM3_5 = crate::Reg<qmem3_5::QMEM3_5_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_5;
#[doc = "QMEM3_6 register accessor: an alias for `Reg<QMEM3_6_SPEC>`"]
pub type QMEM3_6 = crate::Reg<qmem3_6::QMEM3_6_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_6;
#[doc = "QMEM3_7 register accessor: an alias for `Reg<QMEM3_7_SPEC>`"]
pub type QMEM3_7 = crate::Reg<qmem3_7::QMEM3_7_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_7;
#[doc = "QMEM3_8 register accessor: an alias for `Reg<QMEM3_8_SPEC>`"]
pub type QMEM3_8 = crate::Reg<qmem3_8::QMEM3_8_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_8;
#[doc = "QMEM3_9 register accessor: an alias for `Reg<QMEM3_9_SPEC>`"]
pub type QMEM3_9 = crate::Reg<qmem3_9::QMEM3_9_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_9;
#[doc = "QMEM3_10 register accessor: an alias for `Reg<QMEM3_10_SPEC>`"]
pub type QMEM3_10 = crate::Reg<qmem3_10::QMEM3_10_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_10;
#[doc = "QMEM3_11 register accessor: an alias for `Reg<QMEM3_11_SPEC>`"]
pub type QMEM3_11 = crate::Reg<qmem3_11::QMEM3_11_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_11;
#[doc = "QMEM3_12 register accessor: an alias for `Reg<QMEM3_12_SPEC>`"]
pub type QMEM3_12 = crate::Reg<qmem3_12::QMEM3_12_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_12;
#[doc = "QMEM3_13 register accessor: an alias for `Reg<QMEM3_13_SPEC>`"]
pub type QMEM3_13 = crate::Reg<qmem3_13::QMEM3_13_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_13;
#[doc = "QMEM3_14 register accessor: an alias for `Reg<QMEM3_14_SPEC>`"]
pub type QMEM3_14 = crate::Reg<qmem3_14::QMEM3_14_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_14;
#[doc = "QMEM3_15 register accessor: an alias for `Reg<QMEM3_15_SPEC>`"]
pub type QMEM3_15 = crate::Reg<qmem3_15::QMEM3_15_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3_15;
#[doc = "HUFFMIN_0 register accessor: an alias for `Reg<HUFFMIN_0_SPEC>`"]
pub type HUFFMIN_0 = crate::Reg<huffmin_0::HUFFMIN_0_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_0;
#[doc = "HUFFMIN_1 register accessor: an alias for `Reg<HUFFMIN_1_SPEC>`"]
pub type HUFFMIN_1 = crate::Reg<huffmin_1::HUFFMIN_1_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_1;
#[doc = "HUFFMIN_2 register accessor: an alias for `Reg<HUFFMIN_2_SPEC>`"]
pub type HUFFMIN_2 = crate::Reg<huffmin_2::HUFFMIN_2_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_2;
#[doc = "HUFFMIN_3 register accessor: an alias for `Reg<HUFFMIN_3_SPEC>`"]
pub type HUFFMIN_3 = crate::Reg<huffmin_3::HUFFMIN_3_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_3;
#[doc = "HUFFMIN_4 register accessor: an alias for `Reg<HUFFMIN_4_SPEC>`"]
pub type HUFFMIN_4 = crate::Reg<huffmin_4::HUFFMIN_4_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_4;
#[doc = "HUFFMIN_5 register accessor: an alias for `Reg<HUFFMIN_5_SPEC>`"]
pub type HUFFMIN_5 = crate::Reg<huffmin_5::HUFFMIN_5_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_5;
#[doc = "HUFFMIN_6 register accessor: an alias for `Reg<HUFFMIN_6_SPEC>`"]
pub type HUFFMIN_6 = crate::Reg<huffmin_6::HUFFMIN_6_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_6;
#[doc = "HUFFMIN_7 register accessor: an alias for `Reg<HUFFMIN_7_SPEC>`"]
pub type HUFFMIN_7 = crate::Reg<huffmin_7::HUFFMIN_7_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_7;
#[doc = "HUFFMIN_8 register accessor: an alias for `Reg<HUFFMIN_8_SPEC>`"]
pub type HUFFMIN_8 = crate::Reg<huffmin_8::HUFFMIN_8_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_8;
#[doc = "HUFFMIN_9 register accessor: an alias for `Reg<HUFFMIN_9_SPEC>`"]
pub type HUFFMIN_9 = crate::Reg<huffmin_9::HUFFMIN_9_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_9;
#[doc = "HUFFMIN_10 register accessor: an alias for `Reg<HUFFMIN_10_SPEC>`"]
pub type HUFFMIN_10 = crate::Reg<huffmin_10::HUFFMIN_10_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_10;
#[doc = "HUFFMIN_11 register accessor: an alias for `Reg<HUFFMIN_11_SPEC>`"]
pub type HUFFMIN_11 = crate::Reg<huffmin_11::HUFFMIN_11_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_11;
#[doc = "HUFFMIN_12 register accessor: an alias for `Reg<HUFFMIN_12_SPEC>`"]
pub type HUFFMIN_12 = crate::Reg<huffmin_12::HUFFMIN_12_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_12;
#[doc = "HUFFMIN_13 register accessor: an alias for `Reg<HUFFMIN_13_SPEC>`"]
pub type HUFFMIN_13 = crate::Reg<huffmin_13::HUFFMIN_13_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_13;
#[doc = "HUFFMIN_14 register accessor: an alias for `Reg<HUFFMIN_14_SPEC>`"]
pub type HUFFMIN_14 = crate::Reg<huffmin_14::HUFFMIN_14_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_14;
#[doc = "HUFFMIN_15 register accessor: an alias for `Reg<HUFFMIN_15_SPEC>`"]
pub type HUFFMIN_15 = crate::Reg<huffmin_15::HUFFMIN_15_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_15;
#[doc = "HUFFBASE0 register accessor: an alias for `Reg<HUFFBASE0_SPEC>`"]
pub type HUFFBASE0 = crate::Reg<huffbase0::HUFFBASE0_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase0;
#[doc = "HUFFBASE1 register accessor: an alias for `Reg<HUFFBASE1_SPEC>`"]
pub type HUFFBASE1 = crate::Reg<huffbase1::HUFFBASE1_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase1;
#[doc = "HUFFBASE2 register accessor: an alias for `Reg<HUFFBASE2_SPEC>`"]
pub type HUFFBASE2 = crate::Reg<huffbase2::HUFFBASE2_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase2;
#[doc = "HUFFBASE3 register accessor: an alias for `Reg<HUFFBASE3_SPEC>`"]
pub type HUFFBASE3 = crate::Reg<huffbase3::HUFFBASE3_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase3;
#[doc = "HUFFBASE4 register accessor: an alias for `Reg<HUFFBASE4_SPEC>`"]
pub type HUFFBASE4 = crate::Reg<huffbase4::HUFFBASE4_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase4;
#[doc = "HUFFBASE5 register accessor: an alias for `Reg<HUFFBASE5_SPEC>`"]
pub type HUFFBASE5 = crate::Reg<huffbase5::HUFFBASE5_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase5;
#[doc = "HUFFBASE6 register accessor: an alias for `Reg<HUFFBASE6_SPEC>`"]
pub type HUFFBASE6 = crate::Reg<huffbase6::HUFFBASE6_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase6;
#[doc = "HUFFBASE7 register accessor: an alias for `Reg<HUFFBASE7_SPEC>`"]
pub type HUFFBASE7 = crate::Reg<huffbase7::HUFFBASE7_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase7;
#[doc = "HUFFBASE8 register accessor: an alias for `Reg<HUFFBASE8_SPEC>`"]
pub type HUFFBASE8 = crate::Reg<huffbase8::HUFFBASE8_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase8;
#[doc = "HUFFBASE9 register accessor: an alias for `Reg<HUFFBASE9_SPEC>`"]
pub type HUFFBASE9 = crate::Reg<huffbase9::HUFFBASE9_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase9;
#[doc = "HUFFBASE10 register accessor: an alias for `Reg<HUFFBASE10_SPEC>`"]
pub type HUFFBASE10 = crate::Reg<huffbase10::HUFFBASE10_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase10;
#[doc = "HUFFBASE11 register accessor: an alias for `Reg<HUFFBASE11_SPEC>`"]
pub type HUFFBASE11 = crate::Reg<huffbase11::HUFFBASE11_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase11;
#[doc = "HUFFBASE12 register accessor: an alias for `Reg<HUFFBASE12_SPEC>`"]
pub type HUFFBASE12 = crate::Reg<huffbase12::HUFFBASE12_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase12;
#[doc = "HUFFBASE13 register accessor: an alias for `Reg<HUFFBASE13_SPEC>`"]
pub type HUFFBASE13 = crate::Reg<huffbase13::HUFFBASE13_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase13;
#[doc = "HUFFBASE14 register accessor: an alias for `Reg<HUFFBASE14_SPEC>`"]
pub type HUFFBASE14 = crate::Reg<huffbase14::HUFFBASE14_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase14;
#[doc = "HUFFBASE15 register accessor: an alias for `Reg<HUFFBASE15_SPEC>`"]
pub type HUFFBASE15 = crate::Reg<huffbase15::HUFFBASE15_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase15;
#[doc = "HUFFBASE16 register accessor: an alias for `Reg<HUFFBASE16_SPEC>`"]
pub type HUFFBASE16 = crate::Reg<huffbase16::HUFFBASE16_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase16;
#[doc = "HUFFBASE17 register accessor: an alias for `Reg<HUFFBASE17_SPEC>`"]
pub type HUFFBASE17 = crate::Reg<huffbase17::HUFFBASE17_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase17;
#[doc = "HUFFBASE18 register accessor: an alias for `Reg<HUFFBASE18_SPEC>`"]
pub type HUFFBASE18 = crate::Reg<huffbase18::HUFFBASE18_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase18;
#[doc = "HUFFBASE19 register accessor: an alias for `Reg<HUFFBASE19_SPEC>`"]
pub type HUFFBASE19 = crate::Reg<huffbase19::HUFFBASE19_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase19;
#[doc = "HUFFBASE20 register accessor: an alias for `Reg<HUFFBASE20_SPEC>`"]
pub type HUFFBASE20 = crate::Reg<huffbase20::HUFFBASE20_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase20;
#[doc = "HUFFBASE21 register accessor: an alias for `Reg<HUFFBASE21_SPEC>`"]
pub type HUFFBASE21 = crate::Reg<huffbase21::HUFFBASE21_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase21;
#[doc = "HUFFBASE22 register accessor: an alias for `Reg<HUFFBASE22_SPEC>`"]
pub type HUFFBASE22 = crate::Reg<huffbase22::HUFFBASE22_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase22;
#[doc = "HUFFBASE23 register accessor: an alias for `Reg<HUFFBASE23_SPEC>`"]
pub type HUFFBASE23 = crate::Reg<huffbase23::HUFFBASE23_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase23;
#[doc = "HUFFBASE24 register accessor: an alias for `Reg<HUFFBASE24_SPEC>`"]
pub type HUFFBASE24 = crate::Reg<huffbase24::HUFFBASE24_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase24;
#[doc = "HUFFBASE25 register accessor: an alias for `Reg<HUFFBASE25_SPEC>`"]
pub type HUFFBASE25 = crate::Reg<huffbase25::HUFFBASE25_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase25;
#[doc = "HUFFBASE26 register accessor: an alias for `Reg<HUFFBASE26_SPEC>`"]
pub type HUFFBASE26 = crate::Reg<huffbase26::HUFFBASE26_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase26;
#[doc = "HUFFBASE27 register accessor: an alias for `Reg<HUFFBASE27_SPEC>`"]
pub type HUFFBASE27 = crate::Reg<huffbase27::HUFFBASE27_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase27;
#[doc = "HUFFBASE28 register accessor: an alias for `Reg<HUFFBASE28_SPEC>`"]
pub type HUFFBASE28 = crate::Reg<huffbase28::HUFFBASE28_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase28;
#[doc = "HUFFBASE29 register accessor: an alias for `Reg<HUFFBASE29_SPEC>`"]
pub type HUFFBASE29 = crate::Reg<huffbase29::HUFFBASE29_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase29;
#[doc = "HUFFBASE30 register accessor: an alias for `Reg<HUFFBASE30_SPEC>`"]
pub type HUFFBASE30 = crate::Reg<huffbase30::HUFFBASE30_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase30;
#[doc = "HUFFBASE31 register accessor: an alias for `Reg<HUFFBASE31_SPEC>`"]
pub type HUFFBASE31 = crate::Reg<huffbase31::HUFFBASE31_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase31;
#[doc = "HUFFSYMB0 register accessor: an alias for `Reg<HUFFSYMB0_SPEC>`"]
pub type HUFFSYMB0 = crate::Reg<huffsymb0::HUFFSYMB0_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb0;
#[doc = "HUFFSYMB1 register accessor: an alias for `Reg<HUFFSYMB1_SPEC>`"]
pub type HUFFSYMB1 = crate::Reg<huffsymb1::HUFFSYMB1_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb1;
#[doc = "HUFFSYMB2 register accessor: an alias for `Reg<HUFFSYMB2_SPEC>`"]
pub type HUFFSYMB2 = crate::Reg<huffsymb2::HUFFSYMB2_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb2;
#[doc = "HUFFSYMB3 register accessor: an alias for `Reg<HUFFSYMB3_SPEC>`"]
pub type HUFFSYMB3 = crate::Reg<huffsymb3::HUFFSYMB3_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb3;
#[doc = "HUFFSYMB4 register accessor: an alias for `Reg<HUFFSYMB4_SPEC>`"]
pub type HUFFSYMB4 = crate::Reg<huffsymb4::HUFFSYMB4_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb4;
#[doc = "HUFFSYMB5 register accessor: an alias for `Reg<HUFFSYMB5_SPEC>`"]
pub type HUFFSYMB5 = crate::Reg<huffsymb5::HUFFSYMB5_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb5;
#[doc = "HUFFSYMB6 register accessor: an alias for `Reg<HUFFSYMB6_SPEC>`"]
pub type HUFFSYMB6 = crate::Reg<huffsymb6::HUFFSYMB6_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb6;
#[doc = "HUFFSYMB7 register accessor: an alias for `Reg<HUFFSYMB7_SPEC>`"]
pub type HUFFSYMB7 = crate::Reg<huffsymb7::HUFFSYMB7_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb7;
#[doc = "HUFFSYMB8 register accessor: an alias for `Reg<HUFFSYMB8_SPEC>`"]
pub type HUFFSYMB8 = crate::Reg<huffsymb8::HUFFSYMB8_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb8;
#[doc = "HUFFSYMB9 register accessor: an alias for `Reg<HUFFSYMB9_SPEC>`"]
pub type HUFFSYMB9 = crate::Reg<huffsymb9::HUFFSYMB9_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb9;
#[doc = "HUFFSYMB10 register accessor: an alias for `Reg<HUFFSYMB10_SPEC>`"]
pub type HUFFSYMB10 = crate::Reg<huffsymb10::HUFFSYMB10_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb10;
#[doc = "HUFFSYMB11 register accessor: an alias for `Reg<HUFFSYMB11_SPEC>`"]
pub type HUFFSYMB11 = crate::Reg<huffsymb11::HUFFSYMB11_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb11;
#[doc = "HUFFSYMB12 register accessor: an alias for `Reg<HUFFSYMB12_SPEC>`"]
pub type HUFFSYMB12 = crate::Reg<huffsymb12::HUFFSYMB12_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb12;
#[doc = "HUFFSYMB13 register accessor: an alias for `Reg<HUFFSYMB13_SPEC>`"]
pub type HUFFSYMB13 = crate::Reg<huffsymb13::HUFFSYMB13_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb13;
#[doc = "HUFFSYMB14 register accessor: an alias for `Reg<HUFFSYMB14_SPEC>`"]
pub type HUFFSYMB14 = crate::Reg<huffsymb14::HUFFSYMB14_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb14;
#[doc = "HUFFSYMB15 register accessor: an alias for `Reg<HUFFSYMB15_SPEC>`"]
pub type HUFFSYMB15 = crate::Reg<huffsymb15::HUFFSYMB15_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb15;
#[doc = "HUFFSYMB16 register accessor: an alias for `Reg<HUFFSYMB16_SPEC>`"]
pub type HUFFSYMB16 = crate::Reg<huffsymb16::HUFFSYMB16_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb16;
#[doc = "HUFFSYMB17 register accessor: an alias for `Reg<HUFFSYMB17_SPEC>`"]
pub type HUFFSYMB17 = crate::Reg<huffsymb17::HUFFSYMB17_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb17;
#[doc = "HUFFSYMB18 register accessor: an alias for `Reg<HUFFSYMB18_SPEC>`"]
pub type HUFFSYMB18 = crate::Reg<huffsymb18::HUFFSYMB18_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb18;
#[doc = "HUFFSYMB19 register accessor: an alias for `Reg<HUFFSYMB19_SPEC>`"]
pub type HUFFSYMB19 = crate::Reg<huffsymb19::HUFFSYMB19_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb19;
#[doc = "HUFFSYMB20 register accessor: an alias for `Reg<HUFFSYMB20_SPEC>`"]
pub type HUFFSYMB20 = crate::Reg<huffsymb20::HUFFSYMB20_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb20;
#[doc = "HUFFSYMB21 register accessor: an alias for `Reg<HUFFSYMB21_SPEC>`"]
pub type HUFFSYMB21 = crate::Reg<huffsymb21::HUFFSYMB21_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb21;
#[doc = "HUFFSYMB22 register accessor: an alias for `Reg<HUFFSYMB22_SPEC>`"]
pub type HUFFSYMB22 = crate::Reg<huffsymb22::HUFFSYMB22_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb22;
#[doc = "HUFFSYMB23 register accessor: an alias for `Reg<HUFFSYMB23_SPEC>`"]
pub type HUFFSYMB23 = crate::Reg<huffsymb23::HUFFSYMB23_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb23;
#[doc = "HUFFSYMB24 register accessor: an alias for `Reg<HUFFSYMB24_SPEC>`"]
pub type HUFFSYMB24 = crate::Reg<huffsymb24::HUFFSYMB24_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb24;
#[doc = "HUFFSYMB25 register accessor: an alias for `Reg<HUFFSYMB25_SPEC>`"]
pub type HUFFSYMB25 = crate::Reg<huffsymb25::HUFFSYMB25_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb25;
#[doc = "HUFFSYMB26 register accessor: an alias for `Reg<HUFFSYMB26_SPEC>`"]
pub type HUFFSYMB26 = crate::Reg<huffsymb26::HUFFSYMB26_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb26;
#[doc = "HUFFSYMB27 register accessor: an alias for `Reg<HUFFSYMB27_SPEC>`"]
pub type HUFFSYMB27 = crate::Reg<huffsymb27::HUFFSYMB27_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb27;
#[doc = "HUFFSYMB28 register accessor: an alias for `Reg<HUFFSYMB28_SPEC>`"]
pub type HUFFSYMB28 = crate::Reg<huffsymb28::HUFFSYMB28_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb28;
#[doc = "HUFFSYMB29 register accessor: an alias for `Reg<HUFFSYMB29_SPEC>`"]
pub type HUFFSYMB29 = crate::Reg<huffsymb29::HUFFSYMB29_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb29;
#[doc = "HUFFSYMB30 register accessor: an alias for `Reg<HUFFSYMB30_SPEC>`"]
pub type HUFFSYMB30 = crate::Reg<huffsymb30::HUFFSYMB30_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb30;
#[doc = "HUFFSYMB31 register accessor: an alias for `Reg<HUFFSYMB31_SPEC>`"]
pub type HUFFSYMB31 = crate::Reg<huffsymb31::HUFFSYMB31_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb31;
#[doc = "HUFFSYMB32 register accessor: an alias for `Reg<HUFFSYMB32_SPEC>`"]
pub type HUFFSYMB32 = crate::Reg<huffsymb32::HUFFSYMB32_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb32;
#[doc = "HUFFSYMB33 register accessor: an alias for `Reg<HUFFSYMB33_SPEC>`"]
pub type HUFFSYMB33 = crate::Reg<huffsymb33::HUFFSYMB33_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb33;
#[doc = "HUFFSYMB34 register accessor: an alias for `Reg<HUFFSYMB34_SPEC>`"]
pub type HUFFSYMB34 = crate::Reg<huffsymb34::HUFFSYMB34_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb34;
#[doc = "HUFFSYMB35 register accessor: an alias for `Reg<HUFFSYMB35_SPEC>`"]
pub type HUFFSYMB35 = crate::Reg<huffsymb35::HUFFSYMB35_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb35;
#[doc = "HUFFSYMB36 register accessor: an alias for `Reg<HUFFSYMB36_SPEC>`"]
pub type HUFFSYMB36 = crate::Reg<huffsymb36::HUFFSYMB36_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb36;
#[doc = "HUFFSYMB37 register accessor: an alias for `Reg<HUFFSYMB37_SPEC>`"]
pub type HUFFSYMB37 = crate::Reg<huffsymb37::HUFFSYMB37_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb37;
#[doc = "HUFFSYMB38 register accessor: an alias for `Reg<HUFFSYMB38_SPEC>`"]
pub type HUFFSYMB38 = crate::Reg<huffsymb38::HUFFSYMB38_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb38;
#[doc = "HUFFSYMB39 register accessor: an alias for `Reg<HUFFSYMB39_SPEC>`"]
pub type HUFFSYMB39 = crate::Reg<huffsymb39::HUFFSYMB39_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb39;
#[doc = "HUFFSYMB40 register accessor: an alias for `Reg<HUFFSYMB40_SPEC>`"]
pub type HUFFSYMB40 = crate::Reg<huffsymb40::HUFFSYMB40_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb40;
#[doc = "HUFFSYMB41 register accessor: an alias for `Reg<HUFFSYMB41_SPEC>`"]
pub type HUFFSYMB41 = crate::Reg<huffsymb41::HUFFSYMB41_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb41;
#[doc = "HUFFSYMB42 register accessor: an alias for `Reg<HUFFSYMB42_SPEC>`"]
pub type HUFFSYMB42 = crate::Reg<huffsymb42::HUFFSYMB42_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb42;
#[doc = "HUFFSYMB43 register accessor: an alias for `Reg<HUFFSYMB43_SPEC>`"]
pub type HUFFSYMB43 = crate::Reg<huffsymb43::HUFFSYMB43_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb43;
#[doc = "HUFFSYMB44 register accessor: an alias for `Reg<HUFFSYMB44_SPEC>`"]
pub type HUFFSYMB44 = crate::Reg<huffsymb44::HUFFSYMB44_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb44;
#[doc = "HUFFSYMB45 register accessor: an alias for `Reg<HUFFSYMB45_SPEC>`"]
pub type HUFFSYMB45 = crate::Reg<huffsymb45::HUFFSYMB45_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb45;
#[doc = "HUFFSYMB46 register accessor: an alias for `Reg<HUFFSYMB46_SPEC>`"]
pub type HUFFSYMB46 = crate::Reg<huffsymb46::HUFFSYMB46_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb46;
#[doc = "HUFFSYMB47 register accessor: an alias for `Reg<HUFFSYMB47_SPEC>`"]
pub type HUFFSYMB47 = crate::Reg<huffsymb47::HUFFSYMB47_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb47;
#[doc = "HUFFSYMB48 register accessor: an alias for `Reg<HUFFSYMB48_SPEC>`"]
pub type HUFFSYMB48 = crate::Reg<huffsymb48::HUFFSYMB48_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb48;
#[doc = "HUFFSYMB49 register accessor: an alias for `Reg<HUFFSYMB49_SPEC>`"]
pub type HUFFSYMB49 = crate::Reg<huffsymb49::HUFFSYMB49_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb49;
#[doc = "HUFFSYMB50 register accessor: an alias for `Reg<HUFFSYMB50_SPEC>`"]
pub type HUFFSYMB50 = crate::Reg<huffsymb50::HUFFSYMB50_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb50;
#[doc = "HUFFSYMB51 register accessor: an alias for `Reg<HUFFSYMB51_SPEC>`"]
pub type HUFFSYMB51 = crate::Reg<huffsymb51::HUFFSYMB51_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb51;
#[doc = "HUFFSYMB52 register accessor: an alias for `Reg<HUFFSYMB52_SPEC>`"]
pub type HUFFSYMB52 = crate::Reg<huffsymb52::HUFFSYMB52_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb52;
#[doc = "HUFFSYMB53 register accessor: an alias for `Reg<HUFFSYMB53_SPEC>`"]
pub type HUFFSYMB53 = crate::Reg<huffsymb53::HUFFSYMB53_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb53;
#[doc = "HUFFSYMB54 register accessor: an alias for `Reg<HUFFSYMB54_SPEC>`"]
pub type HUFFSYMB54 = crate::Reg<huffsymb54::HUFFSYMB54_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb54;
#[doc = "HUFFSYMB55 register accessor: an alias for `Reg<HUFFSYMB55_SPEC>`"]
pub type HUFFSYMB55 = crate::Reg<huffsymb55::HUFFSYMB55_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb55;
#[doc = "HUFFSYMB56 register accessor: an alias for `Reg<HUFFSYMB56_SPEC>`"]
pub type HUFFSYMB56 = crate::Reg<huffsymb56::HUFFSYMB56_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb56;
#[doc = "HUFFSYMB57 register accessor: an alias for `Reg<HUFFSYMB57_SPEC>`"]
pub type HUFFSYMB57 = crate::Reg<huffsymb57::HUFFSYMB57_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb57;
#[doc = "HUFFSYMB58 register accessor: an alias for `Reg<HUFFSYMB58_SPEC>`"]
pub type HUFFSYMB58 = crate::Reg<huffsymb58::HUFFSYMB58_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb58;
#[doc = "HUFFSYMB59 register accessor: an alias for `Reg<HUFFSYMB59_SPEC>`"]
pub type HUFFSYMB59 = crate::Reg<huffsymb59::HUFFSYMB59_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb59;
#[doc = "HUFFSYMB60 register accessor: an alias for `Reg<HUFFSYMB60_SPEC>`"]
pub type HUFFSYMB60 = crate::Reg<huffsymb60::HUFFSYMB60_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb60;
#[doc = "HUFFSYMB61 register accessor: an alias for `Reg<HUFFSYMB61_SPEC>`"]
pub type HUFFSYMB61 = crate::Reg<huffsymb61::HUFFSYMB61_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb61;
#[doc = "HUFFSYMB62 register accessor: an alias for `Reg<HUFFSYMB62_SPEC>`"]
pub type HUFFSYMB62 = crate::Reg<huffsymb62::HUFFSYMB62_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb62;
#[doc = "HUFFSYMB63 register accessor: an alias for `Reg<HUFFSYMB63_SPEC>`"]
pub type HUFFSYMB63 = crate::Reg<huffsymb63::HUFFSYMB63_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb63;
#[doc = "HUFFSYMB64 register accessor: an alias for `Reg<HUFFSYMB64_SPEC>`"]
pub type HUFFSYMB64 = crate::Reg<huffsymb64::HUFFSYMB64_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb64;
#[doc = "HUFFSYMB65 register accessor: an alias for `Reg<HUFFSYMB65_SPEC>`"]
pub type HUFFSYMB65 = crate::Reg<huffsymb65::HUFFSYMB65_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb65;
#[doc = "HUFFSYMB66 register accessor: an alias for `Reg<HUFFSYMB66_SPEC>`"]
pub type HUFFSYMB66 = crate::Reg<huffsymb66::HUFFSYMB66_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb66;
#[doc = "HUFFSYMB67 register accessor: an alias for `Reg<HUFFSYMB67_SPEC>`"]
pub type HUFFSYMB67 = crate::Reg<huffsymb67::HUFFSYMB67_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb67;
#[doc = "HUFFSYMB68 register accessor: an alias for `Reg<HUFFSYMB68_SPEC>`"]
pub type HUFFSYMB68 = crate::Reg<huffsymb68::HUFFSYMB68_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb68;
#[doc = "HUFFSYMB69 register accessor: an alias for `Reg<HUFFSYMB69_SPEC>`"]
pub type HUFFSYMB69 = crate::Reg<huffsymb69::HUFFSYMB69_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb69;
#[doc = "HUFFSYMB70 register accessor: an alias for `Reg<HUFFSYMB70_SPEC>`"]
pub type HUFFSYMB70 = crate::Reg<huffsymb70::HUFFSYMB70_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb70;
#[doc = "HUFFSYMB71 register accessor: an alias for `Reg<HUFFSYMB71_SPEC>`"]
pub type HUFFSYMB71 = crate::Reg<huffsymb71::HUFFSYMB71_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb71;
#[doc = "HUFFSYMB72 register accessor: an alias for `Reg<HUFFSYMB72_SPEC>`"]
pub type HUFFSYMB72 = crate::Reg<huffsymb72::HUFFSYMB72_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb72;
#[doc = "HUFFSYMB73 register accessor: an alias for `Reg<HUFFSYMB73_SPEC>`"]
pub type HUFFSYMB73 = crate::Reg<huffsymb73::HUFFSYMB73_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb73;
#[doc = "HUFFSYMB74 register accessor: an alias for `Reg<HUFFSYMB74_SPEC>`"]
pub type HUFFSYMB74 = crate::Reg<huffsymb74::HUFFSYMB74_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb74;
#[doc = "HUFFSYMB75 register accessor: an alias for `Reg<HUFFSYMB75_SPEC>`"]
pub type HUFFSYMB75 = crate::Reg<huffsymb75::HUFFSYMB75_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb75;
#[doc = "HUFFSYMB76 register accessor: an alias for `Reg<HUFFSYMB76_SPEC>`"]
pub type HUFFSYMB76 = crate::Reg<huffsymb76::HUFFSYMB76_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb76;
#[doc = "HUFFSYMB77 register accessor: an alias for `Reg<HUFFSYMB77_SPEC>`"]
pub type HUFFSYMB77 = crate::Reg<huffsymb77::HUFFSYMB77_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb77;
#[doc = "HUFFSYMB78 register accessor: an alias for `Reg<HUFFSYMB78_SPEC>`"]
pub type HUFFSYMB78 = crate::Reg<huffsymb78::HUFFSYMB78_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb78;
#[doc = "HUFFSYMB79 register accessor: an alias for `Reg<HUFFSYMB79_SPEC>`"]
pub type HUFFSYMB79 = crate::Reg<huffsymb79::HUFFSYMB79_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb79;
#[doc = "HUFFSYMB80 register accessor: an alias for `Reg<HUFFSYMB80_SPEC>`"]
pub type HUFFSYMB80 = crate::Reg<huffsymb80::HUFFSYMB80_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb80;
#[doc = "HUFFSYMB81 register accessor: an alias for `Reg<HUFFSYMB81_SPEC>`"]
pub type HUFFSYMB81 = crate::Reg<huffsymb81::HUFFSYMB81_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb81;
#[doc = "HUFFSYMB82 register accessor: an alias for `Reg<HUFFSYMB82_SPEC>`"]
pub type HUFFSYMB82 = crate::Reg<huffsymb82::HUFFSYMB82_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb82;
#[doc = "HUFFSYMB83 register accessor: an alias for `Reg<HUFFSYMB83_SPEC>`"]
pub type HUFFSYMB83 = crate::Reg<huffsymb83::HUFFSYMB83_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb83;
#[doc = "DHTMEM0 register accessor: an alias for `Reg<DHTMEM0_SPEC>`"]
pub type DHTMEM0 = crate::Reg<dhtmem0::DHTMEM0_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem0;
#[doc = "DHTMEM2 register accessor: an alias for `Reg<DHTMEM2_SPEC>`"]
pub type DHTMEM2 = crate::Reg<dhtmem2::DHTMEM2_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem2;
#[doc = "DHTMEM3 register accessor: an alias for `Reg<DHTMEM3_SPEC>`"]
pub type DHTMEM3 = crate::Reg<dhtmem3::DHTMEM3_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem3;
#[doc = "DHTMEM4 register accessor: an alias for `Reg<DHTMEM4_SPEC>`"]
pub type DHTMEM4 = crate::Reg<dhtmem4::DHTMEM4_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem4;
#[doc = "DHTMEM5 register accessor: an alias for `Reg<DHTMEM5_SPEC>`"]
pub type DHTMEM5 = crate::Reg<dhtmem5::DHTMEM5_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem5;
#[doc = "DHTMEM6 register accessor: an alias for `Reg<DHTMEM6_SPEC>`"]
pub type DHTMEM6 = crate::Reg<dhtmem6::DHTMEM6_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem6;
#[doc = "DHTMEM7 register accessor: an alias for `Reg<DHTMEM7_SPEC>`"]
pub type DHTMEM7 = crate::Reg<dhtmem7::DHTMEM7_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem7;
#[doc = "DHTMEM8 register accessor: an alias for `Reg<DHTMEM8_SPEC>`"]
pub type DHTMEM8 = crate::Reg<dhtmem8::DHTMEM8_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem8;
#[doc = "DHTMEM9 register accessor: an alias for `Reg<DHTMEM9_SPEC>`"]
pub type DHTMEM9 = crate::Reg<dhtmem9::DHTMEM9_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem9;
#[doc = "DHTMEM10 register accessor: an alias for `Reg<DHTMEM10_SPEC>`"]
pub type DHTMEM10 = crate::Reg<dhtmem10::DHTMEM10_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem10;
#[doc = "DHTMEM11 register accessor: an alias for `Reg<DHTMEM11_SPEC>`"]
pub type DHTMEM11 = crate::Reg<dhtmem11::DHTMEM11_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem11;
#[doc = "DHTMEM12 register accessor: an alias for `Reg<DHTMEM12_SPEC>`"]
pub type DHTMEM12 = crate::Reg<dhtmem12::DHTMEM12_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem12;
#[doc = "DHTMEM13 register accessor: an alias for `Reg<DHTMEM13_SPEC>`"]
pub type DHTMEM13 = crate::Reg<dhtmem13::DHTMEM13_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem13;
#[doc = "DHTMEM14 register accessor: an alias for `Reg<DHTMEM14_SPEC>`"]
pub type DHTMEM14 = crate::Reg<dhtmem14::DHTMEM14_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem14;
#[doc = "DHTMEM15 register accessor: an alias for `Reg<DHTMEM15_SPEC>`"]
pub type DHTMEM15 = crate::Reg<dhtmem15::DHTMEM15_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem15;
#[doc = "DHTMEM16 register accessor: an alias for `Reg<DHTMEM16_SPEC>`"]
pub type DHTMEM16 = crate::Reg<dhtmem16::DHTMEM16_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem16;
#[doc = "DHTMEM17 register accessor: an alias for `Reg<DHTMEM17_SPEC>`"]
pub type DHTMEM17 = crate::Reg<dhtmem17::DHTMEM17_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem17;
#[doc = "DHTMEM18 register accessor: an alias for `Reg<DHTMEM18_SPEC>`"]
pub type DHTMEM18 = crate::Reg<dhtmem18::DHTMEM18_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem18;
#[doc = "DHTMEM19 register accessor: an alias for `Reg<DHTMEM19_SPEC>`"]
pub type DHTMEM19 = crate::Reg<dhtmem19::DHTMEM19_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem19;
#[doc = "DHTMEM20 register accessor: an alias for `Reg<DHTMEM20_SPEC>`"]
pub type DHTMEM20 = crate::Reg<dhtmem20::DHTMEM20_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem20;
#[doc = "DHTMEM21 register accessor: an alias for `Reg<DHTMEM21_SPEC>`"]
pub type DHTMEM21 = crate::Reg<dhtmem21::DHTMEM21_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem21;
#[doc = "DHTMEM22 register accessor: an alias for `Reg<DHTMEM22_SPEC>`"]
pub type DHTMEM22 = crate::Reg<dhtmem22::DHTMEM22_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem22;
#[doc = "DHTMEM23 register accessor: an alias for `Reg<DHTMEM23_SPEC>`"]
pub type DHTMEM23 = crate::Reg<dhtmem23::DHTMEM23_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem23;
#[doc = "DHTMEM24 register accessor: an alias for `Reg<DHTMEM24_SPEC>`"]
pub type DHTMEM24 = crate::Reg<dhtmem24::DHTMEM24_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem24;
#[doc = "DHTMEM25 register accessor: an alias for `Reg<DHTMEM25_SPEC>`"]
pub type DHTMEM25 = crate::Reg<dhtmem25::DHTMEM25_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem25;
#[doc = "DHTMEM26 register accessor: an alias for `Reg<DHTMEM26_SPEC>`"]
pub type DHTMEM26 = crate::Reg<dhtmem26::DHTMEM26_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem26;
#[doc = "DHTMEM27 register accessor: an alias for `Reg<DHTMEM27_SPEC>`"]
pub type DHTMEM27 = crate::Reg<dhtmem27::DHTMEM27_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem27;
#[doc = "DHTMEM28 register accessor: an alias for `Reg<DHTMEM28_SPEC>`"]
pub type DHTMEM28 = crate::Reg<dhtmem28::DHTMEM28_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem28;
#[doc = "DHTMEM29 register accessor: an alias for `Reg<DHTMEM29_SPEC>`"]
pub type DHTMEM29 = crate::Reg<dhtmem29::DHTMEM29_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem29;
#[doc = "DHTMEM30 register accessor: an alias for `Reg<DHTMEM30_SPEC>`"]
pub type DHTMEM30 = crate::Reg<dhtmem30::DHTMEM30_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem30;
#[doc = "DHTMEM31 register accessor: an alias for `Reg<DHTMEM31_SPEC>`"]
pub type DHTMEM31 = crate::Reg<dhtmem31::DHTMEM31_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem31;
#[doc = "DHTMEM32 register accessor: an alias for `Reg<DHTMEM32_SPEC>`"]
pub type DHTMEM32 = crate::Reg<dhtmem32::DHTMEM32_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem32;
#[doc = "DHTMEM33 register accessor: an alias for `Reg<DHTMEM33_SPEC>`"]
pub type DHTMEM33 = crate::Reg<dhtmem33::DHTMEM33_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem33;
#[doc = "DHTMEM34 register accessor: an alias for `Reg<DHTMEM34_SPEC>`"]
pub type DHTMEM34 = crate::Reg<dhtmem34::DHTMEM34_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem34;
#[doc = "DHTMEM35 register accessor: an alias for `Reg<DHTMEM35_SPEC>`"]
pub type DHTMEM35 = crate::Reg<dhtmem35::DHTMEM35_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem35;
#[doc = "DHTMEM36 register accessor: an alias for `Reg<DHTMEM36_SPEC>`"]
pub type DHTMEM36 = crate::Reg<dhtmem36::DHTMEM36_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem36;
#[doc = "DHTMEM37 register accessor: an alias for `Reg<DHTMEM37_SPEC>`"]
pub type DHTMEM37 = crate::Reg<dhtmem37::DHTMEM37_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem37;
#[doc = "DHTMEM38 register accessor: an alias for `Reg<DHTMEM38_SPEC>`"]
pub type DHTMEM38 = crate::Reg<dhtmem38::DHTMEM38_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem38;
#[doc = "DHTMEM39 register accessor: an alias for `Reg<DHTMEM39_SPEC>`"]
pub type DHTMEM39 = crate::Reg<dhtmem39::DHTMEM39_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem39;
#[doc = "DHTMEM40 register accessor: an alias for `Reg<DHTMEM40_SPEC>`"]
pub type DHTMEM40 = crate::Reg<dhtmem40::DHTMEM40_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem40;
#[doc = "DHTMEM41 register accessor: an alias for `Reg<DHTMEM41_SPEC>`"]
pub type DHTMEM41 = crate::Reg<dhtmem41::DHTMEM41_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem41;
#[doc = "DHTMEM42 register accessor: an alias for `Reg<DHTMEM42_SPEC>`"]
pub type DHTMEM42 = crate::Reg<dhtmem42::DHTMEM42_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem42;
#[doc = "DHTMEM43 register accessor: an alias for `Reg<DHTMEM43_SPEC>`"]
pub type DHTMEM43 = crate::Reg<dhtmem43::DHTMEM43_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem43;
#[doc = "DHTMEM44 register accessor: an alias for `Reg<DHTMEM44_SPEC>`"]
pub type DHTMEM44 = crate::Reg<dhtmem44::DHTMEM44_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem44;
#[doc = "DHTMEM45 register accessor: an alias for `Reg<DHTMEM45_SPEC>`"]
pub type DHTMEM45 = crate::Reg<dhtmem45::DHTMEM45_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem45;
#[doc = "DHTMEM46 register accessor: an alias for `Reg<DHTMEM46_SPEC>`"]
pub type DHTMEM46 = crate::Reg<dhtmem46::DHTMEM46_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem46;
#[doc = "DHTMEM47 register accessor: an alias for `Reg<DHTMEM47_SPEC>`"]
pub type DHTMEM47 = crate::Reg<dhtmem47::DHTMEM47_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem47;
#[doc = "DHTMEM48 register accessor: an alias for `Reg<DHTMEM48_SPEC>`"]
pub type DHTMEM48 = crate::Reg<dhtmem48::DHTMEM48_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem48;
#[doc = "DHTMEM49 register accessor: an alias for `Reg<DHTMEM49_SPEC>`"]
pub type DHTMEM49 = crate::Reg<dhtmem49::DHTMEM49_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem49;
#[doc = "DHTMEM50 register accessor: an alias for `Reg<DHTMEM50_SPEC>`"]
pub type DHTMEM50 = crate::Reg<dhtmem50::DHTMEM50_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem50;
#[doc = "DHTMEM51 register accessor: an alias for `Reg<DHTMEM51_SPEC>`"]
pub type DHTMEM51 = crate::Reg<dhtmem51::DHTMEM51_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem51;
#[doc = "DHTMEM52 register accessor: an alias for `Reg<DHTMEM52_SPEC>`"]
pub type DHTMEM52 = crate::Reg<dhtmem52::DHTMEM52_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem52;
#[doc = "DHTMEM53 register accessor: an alias for `Reg<DHTMEM53_SPEC>`"]
pub type DHTMEM53 = crate::Reg<dhtmem53::DHTMEM53_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem53;
#[doc = "DHTMEM54 register accessor: an alias for `Reg<DHTMEM54_SPEC>`"]
pub type DHTMEM54 = crate::Reg<dhtmem54::DHTMEM54_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem54;
#[doc = "DHTMEM55 register accessor: an alias for `Reg<DHTMEM55_SPEC>`"]
pub type DHTMEM55 = crate::Reg<dhtmem55::DHTMEM55_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem55;
#[doc = "DHTMEM56 register accessor: an alias for `Reg<DHTMEM56_SPEC>`"]
pub type DHTMEM56 = crate::Reg<dhtmem56::DHTMEM56_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem56;
#[doc = "DHTMEM57 register accessor: an alias for `Reg<DHTMEM57_SPEC>`"]
pub type DHTMEM57 = crate::Reg<dhtmem57::DHTMEM57_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem57;
#[doc = "DHTMEM58 register accessor: an alias for `Reg<DHTMEM58_SPEC>`"]
pub type DHTMEM58 = crate::Reg<dhtmem58::DHTMEM58_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem58;
#[doc = "DHTMEM59 register accessor: an alias for `Reg<DHTMEM59_SPEC>`"]
pub type DHTMEM59 = crate::Reg<dhtmem59::DHTMEM59_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem59;
#[doc = "DHTMEM60 register accessor: an alias for `Reg<DHTMEM60_SPEC>`"]
pub type DHTMEM60 = crate::Reg<dhtmem60::DHTMEM60_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem60;
#[doc = "DHTMEM61 register accessor: an alias for `Reg<DHTMEM61_SPEC>`"]
pub type DHTMEM61 = crate::Reg<dhtmem61::DHTMEM61_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem61;
#[doc = "DHTMEM62 register accessor: an alias for `Reg<DHTMEM62_SPEC>`"]
pub type DHTMEM62 = crate::Reg<dhtmem62::DHTMEM62_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem62;
#[doc = "DHTMEM63 register accessor: an alias for `Reg<DHTMEM63_SPEC>`"]
pub type DHTMEM63 = crate::Reg<dhtmem63::DHTMEM63_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem63;
#[doc = "DHTMEM64 register accessor: an alias for `Reg<DHTMEM64_SPEC>`"]
pub type DHTMEM64 = crate::Reg<dhtmem64::DHTMEM64_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem64;
#[doc = "DHTMEM65 register accessor: an alias for `Reg<DHTMEM65_SPEC>`"]
pub type DHTMEM65 = crate::Reg<dhtmem65::DHTMEM65_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem65;
#[doc = "DHTMEM66 register accessor: an alias for `Reg<DHTMEM66_SPEC>`"]
pub type DHTMEM66 = crate::Reg<dhtmem66::DHTMEM66_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem66;
#[doc = "DHTMEM67 register accessor: an alias for `Reg<DHTMEM67_SPEC>`"]
pub type DHTMEM67 = crate::Reg<dhtmem67::DHTMEM67_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem67;
#[doc = "DHTMEM68 register accessor: an alias for `Reg<DHTMEM68_SPEC>`"]
pub type DHTMEM68 = crate::Reg<dhtmem68::DHTMEM68_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem68;
#[doc = "DHTMEM69 register accessor: an alias for `Reg<DHTMEM69_SPEC>`"]
pub type DHTMEM69 = crate::Reg<dhtmem69::DHTMEM69_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem69;
#[doc = "DHTMEM70 register accessor: an alias for `Reg<DHTMEM70_SPEC>`"]
pub type DHTMEM70 = crate::Reg<dhtmem70::DHTMEM70_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem70;
#[doc = "DHTMEM71 register accessor: an alias for `Reg<DHTMEM71_SPEC>`"]
pub type DHTMEM71 = crate::Reg<dhtmem71::DHTMEM71_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem71;
#[doc = "DHTMEM72 register accessor: an alias for `Reg<DHTMEM72_SPEC>`"]
pub type DHTMEM72 = crate::Reg<dhtmem72::DHTMEM72_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem72;
#[doc = "DHTMEM73 register accessor: an alias for `Reg<DHTMEM73_SPEC>`"]
pub type DHTMEM73 = crate::Reg<dhtmem73::DHTMEM73_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem73;
#[doc = "DHTMEM74 register accessor: an alias for `Reg<DHTMEM74_SPEC>`"]
pub type DHTMEM74 = crate::Reg<dhtmem74::DHTMEM74_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem74;
#[doc = "DHTMEM75 register accessor: an alias for `Reg<DHTMEM75_SPEC>`"]
pub type DHTMEM75 = crate::Reg<dhtmem75::DHTMEM75_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem75;
#[doc = "DHTMEM76 register accessor: an alias for `Reg<DHTMEM76_SPEC>`"]
pub type DHTMEM76 = crate::Reg<dhtmem76::DHTMEM76_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem76;
#[doc = "DHTMEM77 register accessor: an alias for `Reg<DHTMEM77_SPEC>`"]
pub type DHTMEM77 = crate::Reg<dhtmem77::DHTMEM77_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem77;
#[doc = "DHTMEM78 register accessor: an alias for `Reg<DHTMEM78_SPEC>`"]
pub type DHTMEM78 = crate::Reg<dhtmem78::DHTMEM78_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem78;
#[doc = "DHTMEM79 register accessor: an alias for `Reg<DHTMEM79_SPEC>`"]
pub type DHTMEM79 = crate::Reg<dhtmem79::DHTMEM79_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem79;
#[doc = "DHTMEM80 register accessor: an alias for `Reg<DHTMEM80_SPEC>`"]
pub type DHTMEM80 = crate::Reg<dhtmem80::DHTMEM80_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem80;
#[doc = "DHTMEM81 register accessor: an alias for `Reg<DHTMEM81_SPEC>`"]
pub type DHTMEM81 = crate::Reg<dhtmem81::DHTMEM81_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem81;
#[doc = "DHTMEM82 register accessor: an alias for `Reg<DHTMEM82_SPEC>`"]
pub type DHTMEM82 = crate::Reg<dhtmem82::DHTMEM82_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem82;
#[doc = "DHTMEM83 register accessor: an alias for `Reg<DHTMEM83_SPEC>`"]
pub type DHTMEM83 = crate::Reg<dhtmem83::DHTMEM83_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem83;
#[doc = "DHTMEM84 register accessor: an alias for `Reg<DHTMEM84_SPEC>`"]
pub type DHTMEM84 = crate::Reg<dhtmem84::DHTMEM84_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem84;
#[doc = "DHTMEM85 register accessor: an alias for `Reg<DHTMEM85_SPEC>`"]
pub type DHTMEM85 = crate::Reg<dhtmem85::DHTMEM85_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem85;
#[doc = "DHTMEM86 register accessor: an alias for `Reg<DHTMEM86_SPEC>`"]
pub type DHTMEM86 = crate::Reg<dhtmem86::DHTMEM86_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem86;
#[doc = "DHTMEM87 register accessor: an alias for `Reg<DHTMEM87_SPEC>`"]
pub type DHTMEM87 = crate::Reg<dhtmem87::DHTMEM87_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem87;
#[doc = "DHTMEM88 register accessor: an alias for `Reg<DHTMEM88_SPEC>`"]
pub type DHTMEM88 = crate::Reg<dhtmem88::DHTMEM88_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem88;
#[doc = "DHTMEM89 register accessor: an alias for `Reg<DHTMEM89_SPEC>`"]
pub type DHTMEM89 = crate::Reg<dhtmem89::DHTMEM89_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem89;
#[doc = "DHTMEM90 register accessor: an alias for `Reg<DHTMEM90_SPEC>`"]
pub type DHTMEM90 = crate::Reg<dhtmem90::DHTMEM90_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem90;
#[doc = "DHTMEM91 register accessor: an alias for `Reg<DHTMEM91_SPEC>`"]
pub type DHTMEM91 = crate::Reg<dhtmem91::DHTMEM91_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem91;
#[doc = "DHTMEM92 register accessor: an alias for `Reg<DHTMEM92_SPEC>`"]
pub type DHTMEM92 = crate::Reg<dhtmem92::DHTMEM92_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem92;
#[doc = "DHTMEM93 register accessor: an alias for `Reg<DHTMEM93_SPEC>`"]
pub type DHTMEM93 = crate::Reg<dhtmem93::DHTMEM93_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem93;
#[doc = "DHTMEM94 register accessor: an alias for `Reg<DHTMEM94_SPEC>`"]
pub type DHTMEM94 = crate::Reg<dhtmem94::DHTMEM94_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem94;
#[doc = "DHTMEM95 register accessor: an alias for `Reg<DHTMEM95_SPEC>`"]
pub type DHTMEM95 = crate::Reg<dhtmem95::DHTMEM95_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem95;
#[doc = "DHTMEM96 register accessor: an alias for `Reg<DHTMEM96_SPEC>`"]
pub type DHTMEM96 = crate::Reg<dhtmem96::DHTMEM96_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem96;
#[doc = "DHTMEM97 register accessor: an alias for `Reg<DHTMEM97_SPEC>`"]
pub type DHTMEM97 = crate::Reg<dhtmem97::DHTMEM97_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem97;
#[doc = "DHTMEM98 register accessor: an alias for `Reg<DHTMEM98_SPEC>`"]
pub type DHTMEM98 = crate::Reg<dhtmem98::DHTMEM98_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem98;
#[doc = "DHTMEM99 register accessor: an alias for `Reg<DHTMEM99_SPEC>`"]
pub type DHTMEM99 = crate::Reg<dhtmem99::DHTMEM99_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem99;
#[doc = "DHTMEM100 register accessor: an alias for `Reg<DHTMEM100_SPEC>`"]
pub type DHTMEM100 = crate::Reg<dhtmem100::DHTMEM100_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem100;
#[doc = "DHTMEM101 register accessor: an alias for `Reg<DHTMEM101_SPEC>`"]
pub type DHTMEM101 = crate::Reg<dhtmem101::DHTMEM101_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem101;
#[doc = "DHTMEM102 register accessor: an alias for `Reg<DHTMEM102_SPEC>`"]
pub type DHTMEM102 = crate::Reg<dhtmem102::DHTMEM102_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem102;
#[doc = "DHTMEM103 register accessor: an alias for `Reg<DHTMEM103_SPEC>`"]
pub type DHTMEM103 = crate::Reg<dhtmem103::DHTMEM103_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem103;
#[doc = "HUFFENC_AC0_0 register accessor: an alias for `Reg<HUFFENC_AC0_0_SPEC>`"]
pub type HUFFENC_AC0_0 = crate::Reg<huffenc_ac0_0::HUFFENC_AC0_0_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_0;
#[doc = "HUFFENC_AC0_1 register accessor: an alias for `Reg<HUFFENC_AC0_1_SPEC>`"]
pub type HUFFENC_AC0_1 = crate::Reg<huffenc_ac0_1::HUFFENC_AC0_1_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_1;
#[doc = "HUFFENC_AC0_2 register accessor: an alias for `Reg<HUFFENC_AC0_2_SPEC>`"]
pub type HUFFENC_AC0_2 = crate::Reg<huffenc_ac0_2::HUFFENC_AC0_2_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_2;
#[doc = "HUFFENC_AC0_3 register accessor: an alias for `Reg<HUFFENC_AC0_3_SPEC>`"]
pub type HUFFENC_AC0_3 = crate::Reg<huffenc_ac0_3::HUFFENC_AC0_3_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_3;
#[doc = "HUFFENC_AC0_4 register accessor: an alias for `Reg<HUFFENC_AC0_4_SPEC>`"]
pub type HUFFENC_AC0_4 = crate::Reg<huffenc_ac0_4::HUFFENC_AC0_4_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_4;
#[doc = "HUFFENC_AC0_5 register accessor: an alias for `Reg<HUFFENC_AC0_5_SPEC>`"]
pub type HUFFENC_AC0_5 = crate::Reg<huffenc_ac0_5::HUFFENC_AC0_5_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_5;
#[doc = "HUFFENC_AC0_6 register accessor: an alias for `Reg<HUFFENC_AC0_6_SPEC>`"]
pub type HUFFENC_AC0_6 = crate::Reg<huffenc_ac0_6::HUFFENC_AC0_6_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_6;
#[doc = "HUFFENC_AC0_7 register accessor: an alias for `Reg<HUFFENC_AC0_7_SPEC>`"]
pub type HUFFENC_AC0_7 = crate::Reg<huffenc_ac0_7::HUFFENC_AC0_7_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_7;
#[doc = "HUFFENC_AC0_8 register accessor: an alias for `Reg<HUFFENC_AC0_8_SPEC>`"]
pub type HUFFENC_AC0_8 = crate::Reg<huffenc_ac0_8::HUFFENC_AC0_8_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_8;
#[doc = "HUFFENC_AC0_9 register accessor: an alias for `Reg<HUFFENC_AC0_9_SPEC>`"]
pub type HUFFENC_AC0_9 = crate::Reg<huffenc_ac0_9::HUFFENC_AC0_9_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_9;
#[doc = "HUFFENC_AC0_10 register accessor: an alias for `Reg<HUFFENC_AC0_10_SPEC>`"]
pub type HUFFENC_AC0_10 = crate::Reg<huffenc_ac0_10::HUFFENC_AC0_10_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_10;
#[doc = "HUFFENC_AC0_11 register accessor: an alias for `Reg<HUFFENC_AC0_11_SPEC>`"]
pub type HUFFENC_AC0_11 = crate::Reg<huffenc_ac0_11::HUFFENC_AC0_11_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_11;
#[doc = "HUFFENC_AC0_12 register accessor: an alias for `Reg<HUFFENC_AC0_12_SPEC>`"]
pub type HUFFENC_AC0_12 = crate::Reg<huffenc_ac0_12::HUFFENC_AC0_12_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_12;
#[doc = "HUFFENC_AC0_13 register accessor: an alias for `Reg<HUFFENC_AC0_13_SPEC>`"]
pub type HUFFENC_AC0_13 = crate::Reg<huffenc_ac0_13::HUFFENC_AC0_13_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_13;
#[doc = "HUFFENC_AC0_14 register accessor: an alias for `Reg<HUFFENC_AC0_14_SPEC>`"]
pub type HUFFENC_AC0_14 = crate::Reg<huffenc_ac0_14::HUFFENC_AC0_14_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_14;
#[doc = "HUFFENC_AC0_15 register accessor: an alias for `Reg<HUFFENC_AC0_15_SPEC>`"]
pub type HUFFENC_AC0_15 = crate::Reg<huffenc_ac0_15::HUFFENC_AC0_15_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_15;
#[doc = "HUFFENC_AC0_16 register accessor: an alias for `Reg<HUFFENC_AC0_16_SPEC>`"]
pub type HUFFENC_AC0_16 = crate::Reg<huffenc_ac0_16::HUFFENC_AC0_16_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_16;
#[doc = "HUFFENC_AC0_17 register accessor: an alias for `Reg<HUFFENC_AC0_17_SPEC>`"]
pub type HUFFENC_AC0_17 = crate::Reg<huffenc_ac0_17::HUFFENC_AC0_17_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_17;
#[doc = "HUFFENC_AC0_18 register accessor: an alias for `Reg<HUFFENC_AC0_18_SPEC>`"]
pub type HUFFENC_AC0_18 = crate::Reg<huffenc_ac0_18::HUFFENC_AC0_18_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_18;
#[doc = "HUFFENC_AC0_19 register accessor: an alias for `Reg<HUFFENC_AC0_19_SPEC>`"]
pub type HUFFENC_AC0_19 = crate::Reg<huffenc_ac0_19::HUFFENC_AC0_19_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_19;
#[doc = "HUFFENC_AC0_20 register accessor: an alias for `Reg<HUFFENC_AC0_20_SPEC>`"]
pub type HUFFENC_AC0_20 = crate::Reg<huffenc_ac0_20::HUFFENC_AC0_20_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_20;
#[doc = "HUFFENC_AC0_21 register accessor: an alias for `Reg<HUFFENC_AC0_21_SPEC>`"]
pub type HUFFENC_AC0_21 = crate::Reg<huffenc_ac0_21::HUFFENC_AC0_21_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_21;
#[doc = "HUFFENC_AC0_22 register accessor: an alias for `Reg<HUFFENC_AC0_22_SPEC>`"]
pub type HUFFENC_AC0_22 = crate::Reg<huffenc_ac0_22::HUFFENC_AC0_22_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_22;
#[doc = "HUFFENC_AC0_23 register accessor: an alias for `Reg<HUFFENC_AC0_23_SPEC>`"]
pub type HUFFENC_AC0_23 = crate::Reg<huffenc_ac0_23::HUFFENC_AC0_23_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_23;
#[doc = "HUFFENC_AC0_24 register accessor: an alias for `Reg<HUFFENC_AC0_24_SPEC>`"]
pub type HUFFENC_AC0_24 = crate::Reg<huffenc_ac0_24::HUFFENC_AC0_24_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_24;
#[doc = "HUFFENC_AC0_25 register accessor: an alias for `Reg<HUFFENC_AC0_25_SPEC>`"]
pub type HUFFENC_AC0_25 = crate::Reg<huffenc_ac0_25::HUFFENC_AC0_25_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_25;
#[doc = "HUFFENC_AC0_26 register accessor: an alias for `Reg<HUFFENC_AC0_26_SPEC>`"]
pub type HUFFENC_AC0_26 = crate::Reg<huffenc_ac0_26::HUFFENC_AC0_26_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_26;
#[doc = "HUFFENC_AC0_27 register accessor: an alias for `Reg<HUFFENC_AC0_27_SPEC>`"]
pub type HUFFENC_AC0_27 = crate::Reg<huffenc_ac0_27::HUFFENC_AC0_27_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_27;
#[doc = "HUFFENC_AC0_28 register accessor: an alias for `Reg<HUFFENC_AC0_28_SPEC>`"]
pub type HUFFENC_AC0_28 = crate::Reg<huffenc_ac0_28::HUFFENC_AC0_28_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_28;
#[doc = "HUFFENC_AC0_29 register accessor: an alias for `Reg<HUFFENC_AC0_29_SPEC>`"]
pub type HUFFENC_AC0_29 = crate::Reg<huffenc_ac0_29::HUFFENC_AC0_29_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_29;
#[doc = "HUFFENC_AC0_30 register accessor: an alias for `Reg<HUFFENC_AC0_30_SPEC>`"]
pub type HUFFENC_AC0_30 = crate::Reg<huffenc_ac0_30::HUFFENC_AC0_30_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_30;
#[doc = "HUFFENC_AC0_31 register accessor: an alias for `Reg<HUFFENC_AC0_31_SPEC>`"]
pub type HUFFENC_AC0_31 = crate::Reg<huffenc_ac0_31::HUFFENC_AC0_31_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_31;
#[doc = "HUFFENC_AC0_32 register accessor: an alias for `Reg<HUFFENC_AC0_32_SPEC>`"]
pub type HUFFENC_AC0_32 = crate::Reg<huffenc_ac0_32::HUFFENC_AC0_32_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_32;
#[doc = "HUFFENC_AC0_33 register accessor: an alias for `Reg<HUFFENC_AC0_33_SPEC>`"]
pub type HUFFENC_AC0_33 = crate::Reg<huffenc_ac0_33::HUFFENC_AC0_33_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_33;
#[doc = "HUFFENC_AC0_34 register accessor: an alias for `Reg<HUFFENC_AC0_34_SPEC>`"]
pub type HUFFENC_AC0_34 = crate::Reg<huffenc_ac0_34::HUFFENC_AC0_34_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_34;
#[doc = "HUFFENC_AC0_35 register accessor: an alias for `Reg<HUFFENC_AC0_35_SPEC>`"]
pub type HUFFENC_AC0_35 = crate::Reg<huffenc_ac0_35::HUFFENC_AC0_35_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_35;
#[doc = "HUFFENC_AC0_36 register accessor: an alias for `Reg<HUFFENC_AC0_36_SPEC>`"]
pub type HUFFENC_AC0_36 = crate::Reg<huffenc_ac0_36::HUFFENC_AC0_36_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_36;
#[doc = "HUFFENC_AC0_37 register accessor: an alias for `Reg<HUFFENC_AC0_37_SPEC>`"]
pub type HUFFENC_AC0_37 = crate::Reg<huffenc_ac0_37::HUFFENC_AC0_37_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_37;
#[doc = "HUFFENC_AC0_38 register accessor: an alias for `Reg<HUFFENC_AC0_38_SPEC>`"]
pub type HUFFENC_AC0_38 = crate::Reg<huffenc_ac0_38::HUFFENC_AC0_38_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_38;
#[doc = "HUFFENC_AC0_39 register accessor: an alias for `Reg<HUFFENC_AC0_39_SPEC>`"]
pub type HUFFENC_AC0_39 = crate::Reg<huffenc_ac0_39::HUFFENC_AC0_39_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_39;
#[doc = "HUFFENC_AC0_40 register accessor: an alias for `Reg<HUFFENC_AC0_40_SPEC>`"]
pub type HUFFENC_AC0_40 = crate::Reg<huffenc_ac0_40::HUFFENC_AC0_40_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_40;
#[doc = "HUFFENC_AC0_41 register accessor: an alias for `Reg<HUFFENC_AC0_41_SPEC>`"]
pub type HUFFENC_AC0_41 = crate::Reg<huffenc_ac0_41::HUFFENC_AC0_41_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_41;
#[doc = "HUFFENC_AC0_42 register accessor: an alias for `Reg<HUFFENC_AC0_42_SPEC>`"]
pub type HUFFENC_AC0_42 = crate::Reg<huffenc_ac0_42::HUFFENC_AC0_42_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_42;
#[doc = "HUFFENC_AC0_43 register accessor: an alias for `Reg<HUFFENC_AC0_43_SPEC>`"]
pub type HUFFENC_AC0_43 = crate::Reg<huffenc_ac0_43::HUFFENC_AC0_43_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_43;
#[doc = "HUFFENC_AC0_44 register accessor: an alias for `Reg<HUFFENC_AC0_44_SPEC>`"]
pub type HUFFENC_AC0_44 = crate::Reg<huffenc_ac0_44::HUFFENC_AC0_44_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_44;
#[doc = "HUFFENC_AC0_45 register accessor: an alias for `Reg<HUFFENC_AC0_45_SPEC>`"]
pub type HUFFENC_AC0_45 = crate::Reg<huffenc_ac0_45::HUFFENC_AC0_45_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_45;
#[doc = "HUFFENC_AC0_46 register accessor: an alias for `Reg<HUFFENC_AC0_46_SPEC>`"]
pub type HUFFENC_AC0_46 = crate::Reg<huffenc_ac0_46::HUFFENC_AC0_46_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_46;
#[doc = "HUFFENC_AC0_47 register accessor: an alias for `Reg<HUFFENC_AC0_47_SPEC>`"]
pub type HUFFENC_AC0_47 = crate::Reg<huffenc_ac0_47::HUFFENC_AC0_47_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_47;
#[doc = "HUFFENC_AC0_48 register accessor: an alias for `Reg<HUFFENC_AC0_48_SPEC>`"]
pub type HUFFENC_AC0_48 = crate::Reg<huffenc_ac0_48::HUFFENC_AC0_48_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_48;
#[doc = "HUFFENC_AC0_49 register accessor: an alias for `Reg<HUFFENC_AC0_49_SPEC>`"]
pub type HUFFENC_AC0_49 = crate::Reg<huffenc_ac0_49::HUFFENC_AC0_49_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_49;
#[doc = "HUFFENC_AC0_50 register accessor: an alias for `Reg<HUFFENC_AC0_50_SPEC>`"]
pub type HUFFENC_AC0_50 = crate::Reg<huffenc_ac0_50::HUFFENC_AC0_50_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_50;
#[doc = "HUFFENC_AC0_51 register accessor: an alias for `Reg<HUFFENC_AC0_51_SPEC>`"]
pub type HUFFENC_AC0_51 = crate::Reg<huffenc_ac0_51::HUFFENC_AC0_51_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_51;
#[doc = "HUFFENC_AC0_52 register accessor: an alias for `Reg<HUFFENC_AC0_52_SPEC>`"]
pub type HUFFENC_AC0_52 = crate::Reg<huffenc_ac0_52::HUFFENC_AC0_52_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_52;
#[doc = "HUFFENC_AC0_53 register accessor: an alias for `Reg<HUFFENC_AC0_53_SPEC>`"]
pub type HUFFENC_AC0_53 = crate::Reg<huffenc_ac0_53::HUFFENC_AC0_53_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_53;
#[doc = "HUFFENC_AC0_54 register accessor: an alias for `Reg<HUFFENC_AC0_54_SPEC>`"]
pub type HUFFENC_AC0_54 = crate::Reg<huffenc_ac0_54::HUFFENC_AC0_54_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_54;
#[doc = "HUFFENC_AC0_55 register accessor: an alias for `Reg<HUFFENC_AC0_55_SPEC>`"]
pub type HUFFENC_AC0_55 = crate::Reg<huffenc_ac0_55::HUFFENC_AC0_55_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_55;
#[doc = "HUFFENC_AC0_56 register accessor: an alias for `Reg<HUFFENC_AC0_56_SPEC>`"]
pub type HUFFENC_AC0_56 = crate::Reg<huffenc_ac0_56::HUFFENC_AC0_56_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_56;
#[doc = "HUFFENC_AC0_57 register accessor: an alias for `Reg<HUFFENC_AC0_57_SPEC>`"]
pub type HUFFENC_AC0_57 = crate::Reg<huffenc_ac0_57::HUFFENC_AC0_57_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_57;
#[doc = "HUFFENC_AC0_58 register accessor: an alias for `Reg<HUFFENC_AC0_58_SPEC>`"]
pub type HUFFENC_AC0_58 = crate::Reg<huffenc_ac0_58::HUFFENC_AC0_58_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_58;
#[doc = "HUFFENC_AC0_59 register accessor: an alias for `Reg<HUFFENC_AC0_59_SPEC>`"]
pub type HUFFENC_AC0_59 = crate::Reg<huffenc_ac0_59::HUFFENC_AC0_59_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_59;
#[doc = "HUFFENC_AC0_60 register accessor: an alias for `Reg<HUFFENC_AC0_60_SPEC>`"]
pub type HUFFENC_AC0_60 = crate::Reg<huffenc_ac0_60::HUFFENC_AC0_60_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_60;
#[doc = "HUFFENC_AC0_61 register accessor: an alias for `Reg<HUFFENC_AC0_61_SPEC>`"]
pub type HUFFENC_AC0_61 = crate::Reg<huffenc_ac0_61::HUFFENC_AC0_61_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_61;
#[doc = "HUFFENC_AC0_62 register accessor: an alias for `Reg<HUFFENC_AC0_62_SPEC>`"]
pub type HUFFENC_AC0_62 = crate::Reg<huffenc_ac0_62::HUFFENC_AC0_62_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_62;
#[doc = "HUFFENC_AC0_63 register accessor: an alias for `Reg<HUFFENC_AC0_63_SPEC>`"]
pub type HUFFENC_AC0_63 = crate::Reg<huffenc_ac0_63::HUFFENC_AC0_63_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_63;
#[doc = "HUFFENC_AC0_64 register accessor: an alias for `Reg<HUFFENC_AC0_64_SPEC>`"]
pub type HUFFENC_AC0_64 = crate::Reg<huffenc_ac0_64::HUFFENC_AC0_64_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_64;
#[doc = "HUFFENC_AC0_65 register accessor: an alias for `Reg<HUFFENC_AC0_65_SPEC>`"]
pub type HUFFENC_AC0_65 = crate::Reg<huffenc_ac0_65::HUFFENC_AC0_65_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_65;
#[doc = "HUFFENC_AC0_66 register accessor: an alias for `Reg<HUFFENC_AC0_66_SPEC>`"]
pub type HUFFENC_AC0_66 = crate::Reg<huffenc_ac0_66::HUFFENC_AC0_66_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_66;
#[doc = "HUFFENC_AC0_67 register accessor: an alias for `Reg<HUFFENC_AC0_67_SPEC>`"]
pub type HUFFENC_AC0_67 = crate::Reg<huffenc_ac0_67::HUFFENC_AC0_67_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_67;
#[doc = "HUFFENC_AC0_68 register accessor: an alias for `Reg<HUFFENC_AC0_68_SPEC>`"]
pub type HUFFENC_AC0_68 = crate::Reg<huffenc_ac0_68::HUFFENC_AC0_68_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_68;
#[doc = "HUFFENC_AC0_69 register accessor: an alias for `Reg<HUFFENC_AC0_69_SPEC>`"]
pub type HUFFENC_AC0_69 = crate::Reg<huffenc_ac0_69::HUFFENC_AC0_69_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_69;
#[doc = "HUFFENC_AC0_70 register accessor: an alias for `Reg<HUFFENC_AC0_70_SPEC>`"]
pub type HUFFENC_AC0_70 = crate::Reg<huffenc_ac0_70::HUFFENC_AC0_70_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_70;
#[doc = "HUFFENC_AC0_71 register accessor: an alias for `Reg<HUFFENC_AC0_71_SPEC>`"]
pub type HUFFENC_AC0_71 = crate::Reg<huffenc_ac0_71::HUFFENC_AC0_71_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_71;
#[doc = "HUFFENC_AC0_72 register accessor: an alias for `Reg<HUFFENC_AC0_72_SPEC>`"]
pub type HUFFENC_AC0_72 = crate::Reg<huffenc_ac0_72::HUFFENC_AC0_72_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_72;
#[doc = "HUFFENC_AC0_73 register accessor: an alias for `Reg<HUFFENC_AC0_73_SPEC>`"]
pub type HUFFENC_AC0_73 = crate::Reg<huffenc_ac0_73::HUFFENC_AC0_73_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_73;
#[doc = "HUFFENC_AC0_74 register accessor: an alias for `Reg<HUFFENC_AC0_74_SPEC>`"]
pub type HUFFENC_AC0_74 = crate::Reg<huffenc_ac0_74::HUFFENC_AC0_74_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_74;
#[doc = "HUFFENC_AC0_75 register accessor: an alias for `Reg<HUFFENC_AC0_75_SPEC>`"]
pub type HUFFENC_AC0_75 = crate::Reg<huffenc_ac0_75::HUFFENC_AC0_75_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_75;
#[doc = "HUFFENC_AC0_76 register accessor: an alias for `Reg<HUFFENC_AC0_76_SPEC>`"]
pub type HUFFENC_AC0_76 = crate::Reg<huffenc_ac0_76::HUFFENC_AC0_76_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_76;
#[doc = "HUFFENC_AC0_77 register accessor: an alias for `Reg<HUFFENC_AC0_77_SPEC>`"]
pub type HUFFENC_AC0_77 = crate::Reg<huffenc_ac0_77::HUFFENC_AC0_77_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_77;
#[doc = "HUFFENC_AC0_78 register accessor: an alias for `Reg<HUFFENC_AC0_78_SPEC>`"]
pub type HUFFENC_AC0_78 = crate::Reg<huffenc_ac0_78::HUFFENC_AC0_78_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_78;
#[doc = "HUFFENC_AC0_79 register accessor: an alias for `Reg<HUFFENC_AC0_79_SPEC>`"]
pub type HUFFENC_AC0_79 = crate::Reg<huffenc_ac0_79::HUFFENC_AC0_79_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_79;
#[doc = "HUFFENC_AC0_80 register accessor: an alias for `Reg<HUFFENC_AC0_80_SPEC>`"]
pub type HUFFENC_AC0_80 = crate::Reg<huffenc_ac0_80::HUFFENC_AC0_80_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_80;
#[doc = "HUFFENC_AC0_81 register accessor: an alias for `Reg<HUFFENC_AC0_81_SPEC>`"]
pub type HUFFENC_AC0_81 = crate::Reg<huffenc_ac0_81::HUFFENC_AC0_81_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_81;
#[doc = "HUFFENC_AC0_82 register accessor: an alias for `Reg<HUFFENC_AC0_82_SPEC>`"]
pub type HUFFENC_AC0_82 = crate::Reg<huffenc_ac0_82::HUFFENC_AC0_82_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_82;
#[doc = "HUFFENC_AC0_83 register accessor: an alias for `Reg<HUFFENC_AC0_83_SPEC>`"]
pub type HUFFENC_AC0_83 = crate::Reg<huffenc_ac0_83::HUFFENC_AC0_83_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_83;
#[doc = "HUFFENC_AC0_84 register accessor: an alias for `Reg<HUFFENC_AC0_84_SPEC>`"]
pub type HUFFENC_AC0_84 = crate::Reg<huffenc_ac0_84::HUFFENC_AC0_84_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_84;
#[doc = "HUFFENC_AC0_85 register accessor: an alias for `Reg<HUFFENC_AC0_85_SPEC>`"]
pub type HUFFENC_AC0_85 = crate::Reg<huffenc_ac0_85::HUFFENC_AC0_85_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_85;
#[doc = "HUFFENC_AC0_86 register accessor: an alias for `Reg<HUFFENC_AC0_86_SPEC>`"]
pub type HUFFENC_AC0_86 = crate::Reg<huffenc_ac0_86::HUFFENC_AC0_86_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_86;
#[doc = "HUFFENC_AC0_87 register accessor: an alias for `Reg<HUFFENC_AC0_87_SPEC>`"]
pub type HUFFENC_AC0_87 = crate::Reg<huffenc_ac0_87::HUFFENC_AC0_87_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_87;
#[doc = "HUFFENC_AC1_0 register accessor: an alias for `Reg<HUFFENC_AC1_0_SPEC>`"]
pub type HUFFENC_AC1_0 = crate::Reg<huffenc_ac1_0::HUFFENC_AC1_0_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_0;
#[doc = "HUFFENC_AC1_1 register accessor: an alias for `Reg<HUFFENC_AC1_1_SPEC>`"]
pub type HUFFENC_AC1_1 = crate::Reg<huffenc_ac1_1::HUFFENC_AC1_1_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_1;
#[doc = "HUFFENC_AC1_2 register accessor: an alias for `Reg<HUFFENC_AC1_2_SPEC>`"]
pub type HUFFENC_AC1_2 = crate::Reg<huffenc_ac1_2::HUFFENC_AC1_2_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_2;
#[doc = "HUFFENC_AC1_3 register accessor: an alias for `Reg<HUFFENC_AC1_3_SPEC>`"]
pub type HUFFENC_AC1_3 = crate::Reg<huffenc_ac1_3::HUFFENC_AC1_3_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_3;
#[doc = "HUFFENC_AC1_4 register accessor: an alias for `Reg<HUFFENC_AC1_4_SPEC>`"]
pub type HUFFENC_AC1_4 = crate::Reg<huffenc_ac1_4::HUFFENC_AC1_4_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_4;
#[doc = "HUFFENC_AC1_5 register accessor: an alias for `Reg<HUFFENC_AC1_5_SPEC>`"]
pub type HUFFENC_AC1_5 = crate::Reg<huffenc_ac1_5::HUFFENC_AC1_5_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_5;
#[doc = "HUFFENC_AC1_6 register accessor: an alias for `Reg<HUFFENC_AC1_6_SPEC>`"]
pub type HUFFENC_AC1_6 = crate::Reg<huffenc_ac1_6::HUFFENC_AC1_6_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_6;
#[doc = "HUFFENC_AC1_7 register accessor: an alias for `Reg<HUFFENC_AC1_7_SPEC>`"]
pub type HUFFENC_AC1_7 = crate::Reg<huffenc_ac1_7::HUFFENC_AC1_7_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_7;
#[doc = "HUFFENC_AC1_8 register accessor: an alias for `Reg<HUFFENC_AC1_8_SPEC>`"]
pub type HUFFENC_AC1_8 = crate::Reg<huffenc_ac1_8::HUFFENC_AC1_8_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_8;
#[doc = "HUFFENC_AC1_9 register accessor: an alias for `Reg<HUFFENC_AC1_9_SPEC>`"]
pub type HUFFENC_AC1_9 = crate::Reg<huffenc_ac1_9::HUFFENC_AC1_9_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_9;
#[doc = "HUFFENC_AC1_10 register accessor: an alias for `Reg<HUFFENC_AC1_10_SPEC>`"]
pub type HUFFENC_AC1_10 = crate::Reg<huffenc_ac1_10::HUFFENC_AC1_10_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_10;
#[doc = "HUFFENC_AC1_11 register accessor: an alias for `Reg<HUFFENC_AC1_11_SPEC>`"]
pub type HUFFENC_AC1_11 = crate::Reg<huffenc_ac1_11::HUFFENC_AC1_11_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_11;
#[doc = "HUFFENC_AC1_12 register accessor: an alias for `Reg<HUFFENC_AC1_12_SPEC>`"]
pub type HUFFENC_AC1_12 = crate::Reg<huffenc_ac1_12::HUFFENC_AC1_12_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_12;
#[doc = "HUFFENC_AC1_13 register accessor: an alias for `Reg<HUFFENC_AC1_13_SPEC>`"]
pub type HUFFENC_AC1_13 = crate::Reg<huffenc_ac1_13::HUFFENC_AC1_13_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_13;
#[doc = "HUFFENC_AC1_14 register accessor: an alias for `Reg<HUFFENC_AC1_14_SPEC>`"]
pub type HUFFENC_AC1_14 = crate::Reg<huffenc_ac1_14::HUFFENC_AC1_14_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_14;
#[doc = "HUFFENC_AC1_15 register accessor: an alias for `Reg<HUFFENC_AC1_15_SPEC>`"]
pub type HUFFENC_AC1_15 = crate::Reg<huffenc_ac1_15::HUFFENC_AC1_15_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_15;
#[doc = "HUFFENC_AC1_16 register accessor: an alias for `Reg<HUFFENC_AC1_16_SPEC>`"]
pub type HUFFENC_AC1_16 = crate::Reg<huffenc_ac1_16::HUFFENC_AC1_16_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_16;
#[doc = "HUFFENC_AC1_17 register accessor: an alias for `Reg<HUFFENC_AC1_17_SPEC>`"]
pub type HUFFENC_AC1_17 = crate::Reg<huffenc_ac1_17::HUFFENC_AC1_17_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_17;
#[doc = "HUFFENC_AC1_18 register accessor: an alias for `Reg<HUFFENC_AC1_18_SPEC>`"]
pub type HUFFENC_AC1_18 = crate::Reg<huffenc_ac1_18::HUFFENC_AC1_18_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_18;
#[doc = "HUFFENC_AC1_19 register accessor: an alias for `Reg<HUFFENC_AC1_19_SPEC>`"]
pub type HUFFENC_AC1_19 = crate::Reg<huffenc_ac1_19::HUFFENC_AC1_19_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_19;
#[doc = "HUFFENC_AC1_20 register accessor: an alias for `Reg<HUFFENC_AC1_20_SPEC>`"]
pub type HUFFENC_AC1_20 = crate::Reg<huffenc_ac1_20::HUFFENC_AC1_20_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_20;
#[doc = "HUFFENC_AC1_21 register accessor: an alias for `Reg<HUFFENC_AC1_21_SPEC>`"]
pub type HUFFENC_AC1_21 = crate::Reg<huffenc_ac1_21::HUFFENC_AC1_21_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_21;
#[doc = "HUFFENC_AC1_22 register accessor: an alias for `Reg<HUFFENC_AC1_22_SPEC>`"]
pub type HUFFENC_AC1_22 = crate::Reg<huffenc_ac1_22::HUFFENC_AC1_22_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_22;
#[doc = "HUFFENC_AC1_23 register accessor: an alias for `Reg<HUFFENC_AC1_23_SPEC>`"]
pub type HUFFENC_AC1_23 = crate::Reg<huffenc_ac1_23::HUFFENC_AC1_23_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_23;
#[doc = "HUFFENC_AC1_24 register accessor: an alias for `Reg<HUFFENC_AC1_24_SPEC>`"]
pub type HUFFENC_AC1_24 = crate::Reg<huffenc_ac1_24::HUFFENC_AC1_24_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_24;
#[doc = "HUFFENC_AC1_25 register accessor: an alias for `Reg<HUFFENC_AC1_25_SPEC>`"]
pub type HUFFENC_AC1_25 = crate::Reg<huffenc_ac1_25::HUFFENC_AC1_25_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_25;
#[doc = "HUFFENC_AC1_26 register accessor: an alias for `Reg<HUFFENC_AC1_26_SPEC>`"]
pub type HUFFENC_AC1_26 = crate::Reg<huffenc_ac1_26::HUFFENC_AC1_26_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_26;
#[doc = "HUFFENC_AC1_27 register accessor: an alias for `Reg<HUFFENC_AC1_27_SPEC>`"]
pub type HUFFENC_AC1_27 = crate::Reg<huffenc_ac1_27::HUFFENC_AC1_27_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_27;
#[doc = "HUFFENC_AC1_28 register accessor: an alias for `Reg<HUFFENC_AC1_28_SPEC>`"]
pub type HUFFENC_AC1_28 = crate::Reg<huffenc_ac1_28::HUFFENC_AC1_28_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_28;
#[doc = "HUFFENC_AC1_29 register accessor: an alias for `Reg<HUFFENC_AC1_29_SPEC>`"]
pub type HUFFENC_AC1_29 = crate::Reg<huffenc_ac1_29::HUFFENC_AC1_29_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_29;
#[doc = "HUFFENC_AC1_30 register accessor: an alias for `Reg<HUFFENC_AC1_30_SPEC>`"]
pub type HUFFENC_AC1_30 = crate::Reg<huffenc_ac1_30::HUFFENC_AC1_30_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_30;
#[doc = "HUFFENC_AC1_31 register accessor: an alias for `Reg<HUFFENC_AC1_31_SPEC>`"]
pub type HUFFENC_AC1_31 = crate::Reg<huffenc_ac1_31::HUFFENC_AC1_31_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_31;
#[doc = "HUFFENC_AC1_32 register accessor: an alias for `Reg<HUFFENC_AC1_32_SPEC>`"]
pub type HUFFENC_AC1_32 = crate::Reg<huffenc_ac1_32::HUFFENC_AC1_32_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_32;
#[doc = "HUFFENC_AC1_33 register accessor: an alias for `Reg<HUFFENC_AC1_33_SPEC>`"]
pub type HUFFENC_AC1_33 = crate::Reg<huffenc_ac1_33::HUFFENC_AC1_33_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_33;
#[doc = "HUFFENC_AC1_34 register accessor: an alias for `Reg<HUFFENC_AC1_34_SPEC>`"]
pub type HUFFENC_AC1_34 = crate::Reg<huffenc_ac1_34::HUFFENC_AC1_34_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_34;
#[doc = "HUFFENC_AC1_35 register accessor: an alias for `Reg<HUFFENC_AC1_35_SPEC>`"]
pub type HUFFENC_AC1_35 = crate::Reg<huffenc_ac1_35::HUFFENC_AC1_35_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_35;
#[doc = "HUFFENC_AC1_36 register accessor: an alias for `Reg<HUFFENC_AC1_36_SPEC>`"]
pub type HUFFENC_AC1_36 = crate::Reg<huffenc_ac1_36::HUFFENC_AC1_36_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_36;
#[doc = "HUFFENC_AC1_37 register accessor: an alias for `Reg<HUFFENC_AC1_37_SPEC>`"]
pub type HUFFENC_AC1_37 = crate::Reg<huffenc_ac1_37::HUFFENC_AC1_37_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_37;
#[doc = "HUFFENC_AC1_38 register accessor: an alias for `Reg<HUFFENC_AC1_38_SPEC>`"]
pub type HUFFENC_AC1_38 = crate::Reg<huffenc_ac1_38::HUFFENC_AC1_38_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_38;
#[doc = "HUFFENC_AC1_39 register accessor: an alias for `Reg<HUFFENC_AC1_39_SPEC>`"]
pub type HUFFENC_AC1_39 = crate::Reg<huffenc_ac1_39::HUFFENC_AC1_39_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_39;
#[doc = "HUFFENC_AC1_40 register accessor: an alias for `Reg<HUFFENC_AC1_40_SPEC>`"]
pub type HUFFENC_AC1_40 = crate::Reg<huffenc_ac1_40::HUFFENC_AC1_40_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_40;
#[doc = "HUFFENC_AC1_41 register accessor: an alias for `Reg<HUFFENC_AC1_41_SPEC>`"]
pub type HUFFENC_AC1_41 = crate::Reg<huffenc_ac1_41::HUFFENC_AC1_41_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_41;
#[doc = "HUFFENC_AC1_42 register accessor: an alias for `Reg<HUFFENC_AC1_42_SPEC>`"]
pub type HUFFENC_AC1_42 = crate::Reg<huffenc_ac1_42::HUFFENC_AC1_42_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_42;
#[doc = "HUFFENC_AC1_43 register accessor: an alias for `Reg<HUFFENC_AC1_43_SPEC>`"]
pub type HUFFENC_AC1_43 = crate::Reg<huffenc_ac1_43::HUFFENC_AC1_43_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_43;
#[doc = "HUFFENC_AC1_44 register accessor: an alias for `Reg<HUFFENC_AC1_44_SPEC>`"]
pub type HUFFENC_AC1_44 = crate::Reg<huffenc_ac1_44::HUFFENC_AC1_44_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_44;
#[doc = "HUFFENC_AC1_45 register accessor: an alias for `Reg<HUFFENC_AC1_45_SPEC>`"]
pub type HUFFENC_AC1_45 = crate::Reg<huffenc_ac1_45::HUFFENC_AC1_45_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_45;
#[doc = "HUFFENC_AC1_46 register accessor: an alias for `Reg<HUFFENC_AC1_46_SPEC>`"]
pub type HUFFENC_AC1_46 = crate::Reg<huffenc_ac1_46::HUFFENC_AC1_46_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_46;
#[doc = "HUFFENC_AC1_47 register accessor: an alias for `Reg<HUFFENC_AC1_47_SPEC>`"]
pub type HUFFENC_AC1_47 = crate::Reg<huffenc_ac1_47::HUFFENC_AC1_47_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_47;
#[doc = "HUFFENC_AC1_48 register accessor: an alias for `Reg<HUFFENC_AC1_48_SPEC>`"]
pub type HUFFENC_AC1_48 = crate::Reg<huffenc_ac1_48::HUFFENC_AC1_48_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_48;
#[doc = "HUFFENC_AC1_49 register accessor: an alias for `Reg<HUFFENC_AC1_49_SPEC>`"]
pub type HUFFENC_AC1_49 = crate::Reg<huffenc_ac1_49::HUFFENC_AC1_49_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_49;
#[doc = "HUFFENC_AC1_50 register accessor: an alias for `Reg<HUFFENC_AC1_50_SPEC>`"]
pub type HUFFENC_AC1_50 = crate::Reg<huffenc_ac1_50::HUFFENC_AC1_50_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_50;
#[doc = "HUFFENC_AC1_51 register accessor: an alias for `Reg<HUFFENC_AC1_51_SPEC>`"]
pub type HUFFENC_AC1_51 = crate::Reg<huffenc_ac1_51::HUFFENC_AC1_51_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_51;
#[doc = "HUFFENC_AC1_52 register accessor: an alias for `Reg<HUFFENC_AC1_52_SPEC>`"]
pub type HUFFENC_AC1_52 = crate::Reg<huffenc_ac1_52::HUFFENC_AC1_52_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_52;
#[doc = "HUFFENC_AC1_53 register accessor: an alias for `Reg<HUFFENC_AC1_53_SPEC>`"]
pub type HUFFENC_AC1_53 = crate::Reg<huffenc_ac1_53::HUFFENC_AC1_53_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_53;
#[doc = "HUFFENC_AC1_54 register accessor: an alias for `Reg<HUFFENC_AC1_54_SPEC>`"]
pub type HUFFENC_AC1_54 = crate::Reg<huffenc_ac1_54::HUFFENC_AC1_54_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_54;
#[doc = "HUFFENC_AC1_55 register accessor: an alias for `Reg<HUFFENC_AC1_55_SPEC>`"]
pub type HUFFENC_AC1_55 = crate::Reg<huffenc_ac1_55::HUFFENC_AC1_55_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_55;
#[doc = "HUFFENC_AC1_56 register accessor: an alias for `Reg<HUFFENC_AC1_56_SPEC>`"]
pub type HUFFENC_AC1_56 = crate::Reg<huffenc_ac1_56::HUFFENC_AC1_56_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_56;
#[doc = "HUFFENC_AC1_57 register accessor: an alias for `Reg<HUFFENC_AC1_57_SPEC>`"]
pub type HUFFENC_AC1_57 = crate::Reg<huffenc_ac1_57::HUFFENC_AC1_57_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_57;
#[doc = "HUFFENC_AC1_58 register accessor: an alias for `Reg<HUFFENC_AC1_58_SPEC>`"]
pub type HUFFENC_AC1_58 = crate::Reg<huffenc_ac1_58::HUFFENC_AC1_58_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_58;
#[doc = "HUFFENC_AC1_59 register accessor: an alias for `Reg<HUFFENC_AC1_59_SPEC>`"]
pub type HUFFENC_AC1_59 = crate::Reg<huffenc_ac1_59::HUFFENC_AC1_59_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_59;
#[doc = "HUFFENC_AC1_60 register accessor: an alias for `Reg<HUFFENC_AC1_60_SPEC>`"]
pub type HUFFENC_AC1_60 = crate::Reg<huffenc_ac1_60::HUFFENC_AC1_60_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_60;
#[doc = "HUFFENC_AC1_61 register accessor: an alias for `Reg<HUFFENC_AC1_61_SPEC>`"]
pub type HUFFENC_AC1_61 = crate::Reg<huffenc_ac1_61::HUFFENC_AC1_61_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_61;
#[doc = "HUFFENC_AC1_62 register accessor: an alias for `Reg<HUFFENC_AC1_62_SPEC>`"]
pub type HUFFENC_AC1_62 = crate::Reg<huffenc_ac1_62::HUFFENC_AC1_62_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_62;
#[doc = "HUFFENC_AC1_63 register accessor: an alias for `Reg<HUFFENC_AC1_63_SPEC>`"]
pub type HUFFENC_AC1_63 = crate::Reg<huffenc_ac1_63::HUFFENC_AC1_63_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_63;
#[doc = "HUFFENC_AC1_64 register accessor: an alias for `Reg<HUFFENC_AC1_64_SPEC>`"]
pub type HUFFENC_AC1_64 = crate::Reg<huffenc_ac1_64::HUFFENC_AC1_64_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_64;
#[doc = "HUFFENC_AC1_65 register accessor: an alias for `Reg<HUFFENC_AC1_65_SPEC>`"]
pub type HUFFENC_AC1_65 = crate::Reg<huffenc_ac1_65::HUFFENC_AC1_65_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_65;
#[doc = "HUFFENC_AC1_66 register accessor: an alias for `Reg<HUFFENC_AC1_66_SPEC>`"]
pub type HUFFENC_AC1_66 = crate::Reg<huffenc_ac1_66::HUFFENC_AC1_66_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_66;
#[doc = "HUFFENC_AC1_67 register accessor: an alias for `Reg<HUFFENC_AC1_67_SPEC>`"]
pub type HUFFENC_AC1_67 = crate::Reg<huffenc_ac1_67::HUFFENC_AC1_67_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_67;
#[doc = "HUFFENC_AC1_68 register accessor: an alias for `Reg<HUFFENC_AC1_68_SPEC>`"]
pub type HUFFENC_AC1_68 = crate::Reg<huffenc_ac1_68::HUFFENC_AC1_68_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_68;
#[doc = "HUFFENC_AC1_69 register accessor: an alias for `Reg<HUFFENC_AC1_69_SPEC>`"]
pub type HUFFENC_AC1_69 = crate::Reg<huffenc_ac1_69::HUFFENC_AC1_69_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_69;
#[doc = "HUFFENC_AC1_70 register accessor: an alias for `Reg<HUFFENC_AC1_70_SPEC>`"]
pub type HUFFENC_AC1_70 = crate::Reg<huffenc_ac1_70::HUFFENC_AC1_70_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_70;
#[doc = "HUFFENC_AC1_71 register accessor: an alias for `Reg<HUFFENC_AC1_71_SPEC>`"]
pub type HUFFENC_AC1_71 = crate::Reg<huffenc_ac1_71::HUFFENC_AC1_71_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_71;
#[doc = "HUFFENC_AC1_72 register accessor: an alias for `Reg<HUFFENC_AC1_72_SPEC>`"]
pub type HUFFENC_AC1_72 = crate::Reg<huffenc_ac1_72::HUFFENC_AC1_72_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_72;
#[doc = "HUFFENC_AC1_73 register accessor: an alias for `Reg<HUFFENC_AC1_73_SPEC>`"]
pub type HUFFENC_AC1_73 = crate::Reg<huffenc_ac1_73::HUFFENC_AC1_73_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_73;
#[doc = "HUFFENC_AC1_74 register accessor: an alias for `Reg<HUFFENC_AC1_74_SPEC>`"]
pub type HUFFENC_AC1_74 = crate::Reg<huffenc_ac1_74::HUFFENC_AC1_74_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_74;
#[doc = "HUFFENC_AC1_75 register accessor: an alias for `Reg<HUFFENC_AC1_75_SPEC>`"]
pub type HUFFENC_AC1_75 = crate::Reg<huffenc_ac1_75::HUFFENC_AC1_75_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_75;
#[doc = "HUFFENC_AC1_76 register accessor: an alias for `Reg<HUFFENC_AC1_76_SPEC>`"]
pub type HUFFENC_AC1_76 = crate::Reg<huffenc_ac1_76::HUFFENC_AC1_76_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_76;
#[doc = "HUFFENC_AC1_77 register accessor: an alias for `Reg<HUFFENC_AC1_77_SPEC>`"]
pub type HUFFENC_AC1_77 = crate::Reg<huffenc_ac1_77::HUFFENC_AC1_77_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_77;
#[doc = "HUFFENC_AC1_78 register accessor: an alias for `Reg<HUFFENC_AC1_78_SPEC>`"]
pub type HUFFENC_AC1_78 = crate::Reg<huffenc_ac1_78::HUFFENC_AC1_78_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_78;
#[doc = "HUFFENC_AC1_79 register accessor: an alias for `Reg<HUFFENC_AC1_79_SPEC>`"]
pub type HUFFENC_AC1_79 = crate::Reg<huffenc_ac1_79::HUFFENC_AC1_79_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_79;
#[doc = "HUFFENC_AC1_80 register accessor: an alias for `Reg<HUFFENC_AC1_80_SPEC>`"]
pub type HUFFENC_AC1_80 = crate::Reg<huffenc_ac1_80::HUFFENC_AC1_80_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_80;
#[doc = "HUFFENC_AC1_81 register accessor: an alias for `Reg<HUFFENC_AC1_81_SPEC>`"]
pub type HUFFENC_AC1_81 = crate::Reg<huffenc_ac1_81::HUFFENC_AC1_81_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_81;
#[doc = "HUFFENC_AC1_82 register accessor: an alias for `Reg<HUFFENC_AC1_82_SPEC>`"]
pub type HUFFENC_AC1_82 = crate::Reg<huffenc_ac1_82::HUFFENC_AC1_82_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_82;
#[doc = "HUFFENC_AC1_83 register accessor: an alias for `Reg<HUFFENC_AC1_83_SPEC>`"]
pub type HUFFENC_AC1_83 = crate::Reg<huffenc_ac1_83::HUFFENC_AC1_83_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_83;
#[doc = "HUFFENC_AC1_84 register accessor: an alias for `Reg<HUFFENC_AC1_84_SPEC>`"]
pub type HUFFENC_AC1_84 = crate::Reg<huffenc_ac1_84::HUFFENC_AC1_84_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_84;
#[doc = "HUFFENC_AC1_85 register accessor: an alias for `Reg<HUFFENC_AC1_85_SPEC>`"]
pub type HUFFENC_AC1_85 = crate::Reg<huffenc_ac1_85::HUFFENC_AC1_85_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_85;
#[doc = "HUFFENC_AC1_86 register accessor: an alias for `Reg<HUFFENC_AC1_86_SPEC>`"]
pub type HUFFENC_AC1_86 = crate::Reg<huffenc_ac1_86::HUFFENC_AC1_86_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_86;
#[doc = "HUFFENC_AC1_87 register accessor: an alias for `Reg<HUFFENC_AC1_87_SPEC>`"]
pub type HUFFENC_AC1_87 = crate::Reg<huffenc_ac1_87::HUFFENC_AC1_87_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_87;
#[doc = "HUFFENC_DC0_0 register accessor: an alias for `Reg<HUFFENC_DC0_0_SPEC>`"]
pub type HUFFENC_DC0_0 = crate::Reg<huffenc_dc0_0::HUFFENC_DC0_0_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_0;
#[doc = "HUFFENC_DC0_1 register accessor: an alias for `Reg<HUFFENC_DC0_1_SPEC>`"]
pub type HUFFENC_DC0_1 = crate::Reg<huffenc_dc0_1::HUFFENC_DC0_1_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_1;
#[doc = "HUFFENC_DC0_2 register accessor: an alias for `Reg<HUFFENC_DC0_2_SPEC>`"]
pub type HUFFENC_DC0_2 = crate::Reg<huffenc_dc0_2::HUFFENC_DC0_2_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_2;
#[doc = "HUFFENC_DC0_3 register accessor: an alias for `Reg<HUFFENC_DC0_3_SPEC>`"]
pub type HUFFENC_DC0_3 = crate::Reg<huffenc_dc0_3::HUFFENC_DC0_3_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_3;
#[doc = "HUFFENC_DC0_4 register accessor: an alias for `Reg<HUFFENC_DC0_4_SPEC>`"]
pub type HUFFENC_DC0_4 = crate::Reg<huffenc_dc0_4::HUFFENC_DC0_4_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_4;
#[doc = "HUFFENC_DC0_5 register accessor: an alias for `Reg<HUFFENC_DC0_5_SPEC>`"]
pub type HUFFENC_DC0_5 = crate::Reg<huffenc_dc0_5::HUFFENC_DC0_5_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_5;
#[doc = "HUFFENC_DC0_6 register accessor: an alias for `Reg<HUFFENC_DC0_6_SPEC>`"]
pub type HUFFENC_DC0_6 = crate::Reg<huffenc_dc0_6::HUFFENC_DC0_6_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_6;
#[doc = "HUFFENC_DC0_7 register accessor: an alias for `Reg<HUFFENC_DC0_7_SPEC>`"]
pub type HUFFENC_DC0_7 = crate::Reg<huffenc_dc0_7::HUFFENC_DC0_7_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_7;
#[doc = "HUFFENC_DC1_0 register accessor: an alias for `Reg<HUFFENC_DC1_0_SPEC>`"]
pub type HUFFENC_DC1_0 = crate::Reg<huffenc_dc1_0::HUFFENC_DC1_0_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_0;
#[doc = "HUFFENC_DC1_1 register accessor: an alias for `Reg<HUFFENC_DC1_1_SPEC>`"]
pub type HUFFENC_DC1_1 = crate::Reg<huffenc_dc1_1::HUFFENC_DC1_1_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_1;
#[doc = "HUFFENC_DC1_2 register accessor: an alias for `Reg<HUFFENC_DC1_2_SPEC>`"]
pub type HUFFENC_DC1_2 = crate::Reg<huffenc_dc1_2::HUFFENC_DC1_2_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_2;
#[doc = "HUFFENC_DC1_3 register accessor: an alias for `Reg<HUFFENC_DC1_3_SPEC>`"]
pub type HUFFENC_DC1_3 = crate::Reg<huffenc_dc1_3::HUFFENC_DC1_3_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_3;
#[doc = "HUFFENC_DC1_4 register accessor: an alias for `Reg<HUFFENC_DC1_4_SPEC>`"]
pub type HUFFENC_DC1_4 = crate::Reg<huffenc_dc1_4::HUFFENC_DC1_4_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_4;
#[doc = "HUFFENC_DC1_5 register accessor: an alias for `Reg<HUFFENC_DC1_5_SPEC>`"]
pub type HUFFENC_DC1_5 = crate::Reg<huffenc_dc1_5::HUFFENC_DC1_5_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_5;
#[doc = "HUFFENC_DC1_6 register accessor: an alias for `Reg<HUFFENC_DC1_6_SPEC>`"]
pub type HUFFENC_DC1_6 = crate::Reg<huffenc_dc1_6::HUFFENC_DC1_6_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_6;
#[doc = "HUFFENC_DC1_7 register accessor: an alias for `Reg<HUFFENC_DC1_7_SPEC>`"]
pub type HUFFENC_DC1_7 = crate::Reg<huffenc_dc1_7::HUFFENC_DC1_7_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_7;
