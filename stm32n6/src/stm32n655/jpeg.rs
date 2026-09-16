#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    confr0: CONFR0,
    confr1: CONFR1,
    confr2: CONFR2,
    confr3: CONFR3,
    confr4: CONFR4,
    confr5: CONFR5,
    confr6: CONFR6,
    confr7: CONFR7,
    _reserved8: [u8; 0x10],
    cr: CR,
    sr: SR,
    cfr: CFR,
    _reserved11: [u8; 0x04],
    dir: DIR,
    dor: DOR,
    _reserved13: [u8; 0x08],
    qmem0_0: QMEM0_0,
    qmem0_1: QMEM0_1,
    qmem0_2: QMEM0_2,
    qmem0_3: QMEM0_3,
    qmem0_4: QMEM0_4,
    qmem0_5: QMEM0_5,
    qmem0_6: QMEM0_6,
    qmem0_7: QMEM0_7,
    qmem0_8: QMEM0_8,
    qmem0_9: QMEM0_9,
    qmem0_10: QMEM0_10,
    qmem0_11: QMEM0_11,
    qmem0_12: QMEM0_12,
    qmem0_13: QMEM0_13,
    qmem0_14: QMEM0_14,
    qmem0_15: QMEM0_15,
    qmem1_0: QMEM1_0,
    qmem1_1: QMEM1_1,
    qmem1_2: QMEM1_2,
    qmem1_3: QMEM1_3,
    qmem1_4: QMEM1_4,
    qmem1_5: QMEM1_5,
    qmem1_6: QMEM1_6,
    qmem1_7: QMEM1_7,
    qmem1_8: QMEM1_8,
    qmem1_9: QMEM1_9,
    qmem1_10: QMEM1_10,
    qmem1_11: QMEM1_11,
    qmem1_12: QMEM1_12,
    qmem1_13: QMEM1_13,
    qmem1_14: QMEM1_14,
    qmem1_15: QMEM1_15,
    qmem2_0: QMEM2_0,
    qmem2_1: QMEM2_1,
    qmem2_2: QMEM2_2,
    qmem2_3: QMEM2_3,
    qmem2_4: QMEM2_4,
    qmem2_5: QMEM2_5,
    qmem2_6: QMEM2_6,
    qmem2_7: QMEM2_7,
    qmem2_8: QMEM2_8,
    qmem2_9: QMEM2_9,
    qmem2_10: QMEM2_10,
    qmem2_11: QMEM2_11,
    qmem2_12: QMEM2_12,
    qmem2_13: QMEM2_13,
    qmem2_14: QMEM2_14,
    qmem2_15: QMEM2_15,
    qmem3_0: QMEM3_0,
    qmem3_1: QMEM3_1,
    qmem3_2: QMEM3_2,
    qmem3_3: QMEM3_3,
    qmem3_4: QMEM3_4,
    qmem3_5: QMEM3_5,
    qmem3_6: QMEM3_6,
    qmem3_7: QMEM3_7,
    qmem3_8: QMEM3_8,
    qmem3_9: QMEM3_9,
    qmem3_10: QMEM3_10,
    qmem3_11: QMEM3_11,
    qmem3_12: QMEM3_12,
    qmem3_13: QMEM3_13,
    qmem3_14: QMEM3_14,
    qmem3_15: QMEM3_15,
    huffmin0_0: HUFFMIN0_0,
    huffmin0_1: HUFFMIN0_1,
    huffmin0_2: HUFFMIN0_2,
    huffmin0_3: HUFFMIN0_3,
    huffmin1_0: HUFFMIN1_0,
    huffmin1_1: HUFFMIN1_1,
    huffmin1_2: HUFFMIN1_2,
    huffmin1_3: HUFFMIN1_3,
    huffmin2_0: HUFFMIN2_0,
    huffmin2_1: HUFFMIN2_1,
    huffmin2_2: HUFFMIN2_2,
    huffmin2_3: HUFFMIN2_3,
    huffmin3_0: HUFFMIN3_0,
    huffmin3_1: HUFFMIN3_1,
    huffmin3_2: HUFFMIN3_2,
    huffmin3_3: HUFFMIN3_3,
    huffbase0: HUFFBASE0,
    huffbase1: HUFFBASE1,
    huffbase2: HUFFBASE2,
    huffbase3: HUFFBASE3,
    huffbase4: HUFFBASE4,
    huffbase5: HUFFBASE5,
    huffbase6: HUFFBASE6,
    huffbase7: HUFFBASE7,
    huffbase8: HUFFBASE8,
    huffbase9: HUFFBASE9,
    huffbase10: HUFFBASE10,
    huffbase11: HUFFBASE11,
    huffbase12: HUFFBASE12,
    huffbase13: HUFFBASE13,
    huffbase14: HUFFBASE14,
    huffbase15: HUFFBASE15,
    huffbase16: HUFFBASE16,
    huffbase17: HUFFBASE17,
    huffbase18: HUFFBASE18,
    huffbase19: HUFFBASE19,
    huffbase20: HUFFBASE20,
    huffbase21: HUFFBASE21,
    huffbase22: HUFFBASE22,
    huffbase23: HUFFBASE23,
    huffbase24: HUFFBASE24,
    huffbase25: HUFFBASE25,
    huffbase26: HUFFBASE26,
    huffbase27: HUFFBASE27,
    huffbase28: HUFFBASE28,
    huffbase29: HUFFBASE29,
    huffbase30: HUFFBASE30,
    huffbase31: HUFFBASE31,
    huffsymb0: HUFFSYMB0,
    huffsymb1: HUFFSYMB1,
    huffsymb2: HUFFSYMB2,
    huffsymb3: HUFFSYMB3,
    huffsymb4: HUFFSYMB4,
    huffsymb5: HUFFSYMB5,
    huffsymb6: HUFFSYMB6,
    huffsymb7: HUFFSYMB7,
    huffsymb8: HUFFSYMB8,
    huffsymb9: HUFFSYMB9,
    huffsymb10: HUFFSYMB10,
    huffsymb11: HUFFSYMB11,
    huffsymb12: HUFFSYMB12,
    huffsymb13: HUFFSYMB13,
    huffsymb14: HUFFSYMB14,
    huffsymb15: HUFFSYMB15,
    huffsymb16: HUFFSYMB16,
    huffsymb17: HUFFSYMB17,
    huffsymb18: HUFFSYMB18,
    huffsymb19: HUFFSYMB19,
    huffsymb20: HUFFSYMB20,
    huffsymb21: HUFFSYMB21,
    huffsymb22: HUFFSYMB22,
    huffsymb23: HUFFSYMB23,
    huffsymb24: HUFFSYMB24,
    huffsymb25: HUFFSYMB25,
    huffsymb26: HUFFSYMB26,
    huffsymb27: HUFFSYMB27,
    huffsymb28: HUFFSYMB28,
    huffsymb29: HUFFSYMB29,
    huffsymb30: HUFFSYMB30,
    huffsymb31: HUFFSYMB31,
    huffsymb32: HUFFSYMB32,
    huffsymb33: HUFFSYMB33,
    huffsymb34: HUFFSYMB34,
    huffsymb35: HUFFSYMB35,
    huffsymb36: HUFFSYMB36,
    huffsymb37: HUFFSYMB37,
    huffsymb38: HUFFSYMB38,
    huffsymb39: HUFFSYMB39,
    huffsymb40: HUFFSYMB40,
    huffsymb41: HUFFSYMB41,
    huffsymb42: HUFFSYMB42,
    huffsymb43: HUFFSYMB43,
    huffsymb44: HUFFSYMB44,
    huffsymb45: HUFFSYMB45,
    huffsymb46: HUFFSYMB46,
    huffsymb47: HUFFSYMB47,
    huffsymb48: HUFFSYMB48,
    huffsymb49: HUFFSYMB49,
    huffsymb50: HUFFSYMB50,
    huffsymb51: HUFFSYMB51,
    huffsymb52: HUFFSYMB52,
    huffsymb53: HUFFSYMB53,
    huffsymb54: HUFFSYMB54,
    huffsymb55: HUFFSYMB55,
    huffsymb56: HUFFSYMB56,
    huffsymb57: HUFFSYMB57,
    huffsymb58: HUFFSYMB58,
    huffsymb59: HUFFSYMB59,
    huffsymb60: HUFFSYMB60,
    huffsymb61: HUFFSYMB61,
    huffsymb62: HUFFSYMB62,
    huffsymb63: HUFFSYMB63,
    huffsymb64: HUFFSYMB64,
    huffsymb65: HUFFSYMB65,
    huffsymb66: HUFFSYMB66,
    huffsymb67: HUFFSYMB67,
    huffsymb68: HUFFSYMB68,
    huffsymb69: HUFFSYMB69,
    huffsymb70: HUFFSYMB70,
    huffsymb71: HUFFSYMB71,
    huffsymb72: HUFFSYMB72,
    huffsymb73: HUFFSYMB73,
    huffsymb74: HUFFSYMB74,
    huffsymb75: HUFFSYMB75,
    huffsymb76: HUFFSYMB76,
    huffsymb77: HUFFSYMB77,
    huffsymb78: HUFFSYMB78,
    huffsymb79: HUFFSYMB79,
    huffsymb80: HUFFSYMB80,
    huffsymb81: HUFFSYMB81,
    huffsymb82: HUFFSYMB82,
    huffsymb83: HUFFSYMB83,
    dhtmem0: DHTMEM0,
    dhtmem1: DHTMEM1,
    dhtmem2: DHTMEM2,
    dhtmem3: DHTMEM3,
    dhtmem4: DHTMEM4,
    dhtmem5: DHTMEM5,
    dhtmem6: DHTMEM6,
    dhtmem7: DHTMEM7,
    dhtmem8: DHTMEM8,
    dhtmem9: DHTMEM9,
    dhtmem10: DHTMEM10,
    dhtmem11: DHTMEM11,
    dhtmem12: DHTMEM12,
    dhtmem13: DHTMEM13,
    dhtmem14: DHTMEM14,
    dhtmem15: DHTMEM15,
    dhtmem16: DHTMEM16,
    dhtmem17: DHTMEM17,
    dhtmem18: DHTMEM18,
    dhtmem19: DHTMEM19,
    dhtmem20: DHTMEM20,
    dhtmem21: DHTMEM21,
    dhtmem22: DHTMEM22,
    dhtmem23: DHTMEM23,
    dhtmem24: DHTMEM24,
    dhtmem25: DHTMEM25,
    dhtmem26: DHTMEM26,
    dhtmem27: DHTMEM27,
    dhtmem28: DHTMEM28,
    dhtmem29: DHTMEM29,
    dhtmem30: DHTMEM30,
    dhtmem31: DHTMEM31,
    dhtmem32: DHTMEM32,
    dhtmem33: DHTMEM33,
    dhtmem34: DHTMEM34,
    dhtmem35: DHTMEM35,
    dhtmem36: DHTMEM36,
    dhtmem37: DHTMEM37,
    dhtmem38: DHTMEM38,
    dhtmem39: DHTMEM39,
    dhtmem40: DHTMEM40,
    dhtmem41: DHTMEM41,
    dhtmem42: DHTMEM42,
    dhtmem43: DHTMEM43,
    dhtmem44: DHTMEM44,
    dhtmem45: DHTMEM45,
    dhtmem46: DHTMEM46,
    dhtmem47: DHTMEM47,
    dhtmem48: DHTMEM48,
    dhtmem49: DHTMEM49,
    dhtmem50: DHTMEM50,
    dhtmem51: DHTMEM51,
    dhtmem52: DHTMEM52,
    dhtmem53: DHTMEM53,
    dhtmem54: DHTMEM54,
    dhtmem55: DHTMEM55,
    dhtmem56: DHTMEM56,
    dhtmem57: DHTMEM57,
    dhtmem58: DHTMEM58,
    dhtmem59: DHTMEM59,
    dhtmem60: DHTMEM60,
    dhtmem61: DHTMEM61,
    dhtmem62: DHTMEM62,
    dhtmem63: DHTMEM63,
    dhtmem64: DHTMEM64,
    dhtmem65: DHTMEM65,
    dhtmem66: DHTMEM66,
    dhtmem67: DHTMEM67,
    dhtmem68: DHTMEM68,
    dhtmem69: DHTMEM69,
    dhtmem70: DHTMEM70,
    dhtmem71: DHTMEM71,
    dhtmem72: DHTMEM72,
    dhtmem73: DHTMEM73,
    dhtmem74: DHTMEM74,
    dhtmem75: DHTMEM75,
    dhtmem76: DHTMEM76,
    dhtmem77: DHTMEM77,
    dhtmem78: DHTMEM78,
    dhtmem79: DHTMEM79,
    dhtmem80: DHTMEM80,
    dhtmem81: DHTMEM81,
    dhtmem82: DHTMEM82,
    dhtmem83: DHTMEM83,
    dhtmem84: DHTMEM84,
    dhtmem85: DHTMEM85,
    dhtmem86: DHTMEM86,
    dhtmem87: DHTMEM87,
    dhtmem88: DHTMEM88,
    dhtmem89: DHTMEM89,
    dhtmem90: DHTMEM90,
    dhtmem91: DHTMEM91,
    dhtmem92: DHTMEM92,
    dhtmem93: DHTMEM93,
    dhtmem94: DHTMEM94,
    dhtmem95: DHTMEM95,
    dhtmem96: DHTMEM96,
    dhtmem97: DHTMEM97,
    dhtmem98: DHTMEM98,
    dhtmem99: DHTMEM99,
    dhtmem100: DHTMEM100,
    dhtmem101: DHTMEM101,
    dhtmem102: DHTMEM102,
    _reserved312: [u8; 0x04],
    huffenc_ac0_0: HUFFENC_AC0_0,
    huffenc_ac0_1: HUFFENC_AC0_1,
    huffenc_ac0_2: HUFFENC_AC0_2,
    huffenc_ac0_3: HUFFENC_AC0_3,
    huffenc_ac0_4: HUFFENC_AC0_4,
    huffenc_ac0_5: HUFFENC_AC0_5,
    huffenc_ac0_6: HUFFENC_AC0_6,
    huffenc_ac0_7: HUFFENC_AC0_7,
    huffenc_ac0_8: HUFFENC_AC0_8,
    huffenc_ac0_9: HUFFENC_AC0_9,
    huffenc_ac0_10: HUFFENC_AC0_10,
    huffenc_ac0_11: HUFFENC_AC0_11,
    huffenc_ac0_12: HUFFENC_AC0_12,
    huffenc_ac0_13: HUFFENC_AC0_13,
    huffenc_ac0_14: HUFFENC_AC0_14,
    huffenc_ac0_15: HUFFENC_AC0_15,
    huffenc_ac0_16: HUFFENC_AC0_16,
    huffenc_ac0_17: HUFFENC_AC0_17,
    huffenc_ac0_18: HUFFENC_AC0_18,
    huffenc_ac0_19: HUFFENC_AC0_19,
    huffenc_ac0_20: HUFFENC_AC0_20,
    huffenc_ac0_21: HUFFENC_AC0_21,
    huffenc_ac0_22: HUFFENC_AC0_22,
    huffenc_ac0_23: HUFFENC_AC0_23,
    huffenc_ac0_24: HUFFENC_AC0_24,
    huffenc_ac0_25: HUFFENC_AC0_25,
    huffenc_ac0_26: HUFFENC_AC0_26,
    huffenc_ac0_27: HUFFENC_AC0_27,
    huffenc_ac0_28: HUFFENC_AC0_28,
    huffenc_ac0_29: HUFFENC_AC0_29,
    huffenc_ac0_30: HUFFENC_AC0_30,
    huffenc_ac0_31: HUFFENC_AC0_31,
    huffenc_ac0_32: HUFFENC_AC0_32,
    huffenc_ac0_33: HUFFENC_AC0_33,
    huffenc_ac0_34: HUFFENC_AC0_34,
    huffenc_ac0_35: HUFFENC_AC0_35,
    huffenc_ac0_36: HUFFENC_AC0_36,
    huffenc_ac0_37: HUFFENC_AC0_37,
    huffenc_ac0_38: HUFFENC_AC0_38,
    huffenc_ac0_39: HUFFENC_AC0_39,
    huffenc_ac0_40: HUFFENC_AC0_40,
    huffenc_ac0_41: HUFFENC_AC0_41,
    huffenc_ac0_42: HUFFENC_AC0_42,
    huffenc_ac0_43: HUFFENC_AC0_43,
    huffenc_ac0_44: HUFFENC_AC0_44,
    huffenc_ac0_45: HUFFENC_AC0_45,
    huffenc_ac0_46: HUFFENC_AC0_46,
    huffenc_ac0_47: HUFFENC_AC0_47,
    huffenc_ac0_48: HUFFENC_AC0_48,
    huffenc_ac0_49: HUFFENC_AC0_49,
    huffenc_ac0_50: HUFFENC_AC0_50,
    huffenc_ac0_51: HUFFENC_AC0_51,
    huffenc_ac0_52: HUFFENC_AC0_52,
    huffenc_ac0_53: HUFFENC_AC0_53,
    huffenc_ac0_54: HUFFENC_AC0_54,
    huffenc_ac0_55: HUFFENC_AC0_55,
    huffenc_ac0_56: HUFFENC_AC0_56,
    huffenc_ac0_57: HUFFENC_AC0_57,
    huffenc_ac0_58: HUFFENC_AC0_58,
    huffenc_ac0_59: HUFFENC_AC0_59,
    huffenc_ac0_60: HUFFENC_AC0_60,
    huffenc_ac0_61: HUFFENC_AC0_61,
    huffenc_ac0_62: HUFFENC_AC0_62,
    huffenc_ac0_63: HUFFENC_AC0_63,
    huffenc_ac0_64: HUFFENC_AC0_64,
    huffenc_ac0_65: HUFFENC_AC0_65,
    huffenc_ac0_66: HUFFENC_AC0_66,
    huffenc_ac0_67: HUFFENC_AC0_67,
    huffenc_ac0_68: HUFFENC_AC0_68,
    huffenc_ac0_69: HUFFENC_AC0_69,
    huffenc_ac0_70: HUFFENC_AC0_70,
    huffenc_ac0_71: HUFFENC_AC0_71,
    huffenc_ac0_72: HUFFENC_AC0_72,
    huffenc_ac0_73: HUFFENC_AC0_73,
    huffenc_ac0_74: HUFFENC_AC0_74,
    huffenc_ac0_75: HUFFENC_AC0_75,
    huffenc_ac0_76: HUFFENC_AC0_76,
    huffenc_ac0_77: HUFFENC_AC0_77,
    huffenc_ac0_78: HUFFENC_AC0_78,
    huffenc_ac0_79: HUFFENC_AC0_79,
    huffenc_ac0_80: HUFFENC_AC0_80,
    huffenc_ac0_81: HUFFENC_AC0_81,
    huffenc_ac0_82: HUFFENC_AC0_82,
    huffenc_ac0_83: HUFFENC_AC0_83,
    huffenc_ac0_84: HUFFENC_AC0_84,
    huffenc_ac0_85: HUFFENC_AC0_85,
    huffenc_ac0_86: HUFFENC_AC0_86,
    huffenc_ac0_87: HUFFENC_AC0_87,
    huffenc_ac1_0: HUFFENC_AC1_0,
    huffenc_ac1_1: HUFFENC_AC1_1,
    huffenc_ac1_2: HUFFENC_AC1_2,
    huffenc_ac1_3: HUFFENC_AC1_3,
    huffenc_ac1_4: HUFFENC_AC1_4,
    huffenc_ac1_5: HUFFENC_AC1_5,
    huffenc_ac1_6: HUFFENC_AC1_6,
    huffenc_ac1_7: HUFFENC_AC1_7,
    huffenc_ac1_8: HUFFENC_AC1_8,
    huffenc_ac1_9: HUFFENC_AC1_9,
    huffenc_ac1_10: HUFFENC_AC1_10,
    huffenc_ac1_11: HUFFENC_AC1_11,
    huffenc_ac1_12: HUFFENC_AC1_12,
    huffenc_ac1_13: HUFFENC_AC1_13,
    huffenc_ac1_14: HUFFENC_AC1_14,
    huffenc_ac1_15: HUFFENC_AC1_15,
    huffenc_ac1_16: HUFFENC_AC1_16,
    huffenc_ac1_17: HUFFENC_AC1_17,
    huffenc_ac1_18: HUFFENC_AC1_18,
    huffenc_ac1_19: HUFFENC_AC1_19,
    huffenc_ac1_20: HUFFENC_AC1_20,
    huffenc_ac1_21: HUFFENC_AC1_21,
    huffenc_ac1_22: HUFFENC_AC1_22,
    huffenc_ac1_23: HUFFENC_AC1_23,
    huffenc_ac1_24: HUFFENC_AC1_24,
    huffenc_ac1_25: HUFFENC_AC1_25,
    huffenc_ac1_26: HUFFENC_AC1_26,
    huffenc_ac1_27: HUFFENC_AC1_27,
    huffenc_ac1_28: HUFFENC_AC1_28,
    huffenc_ac1_29: HUFFENC_AC1_29,
    huffenc_ac1_30: HUFFENC_AC1_30,
    huffenc_ac1_31: HUFFENC_AC1_31,
    huffenc_ac1_32: HUFFENC_AC1_32,
    huffenc_ac1_33: HUFFENC_AC1_33,
    huffenc_ac1_34: HUFFENC_AC1_34,
    huffenc_ac1_35: HUFFENC_AC1_35,
    huffenc_ac1_36: HUFFENC_AC1_36,
    huffenc_ac1_37: HUFFENC_AC1_37,
    huffenc_ac1_38: HUFFENC_AC1_38,
    huffenc_ac1_39: HUFFENC_AC1_39,
    huffenc_ac1_40: HUFFENC_AC1_40,
    huffenc_ac1_41: HUFFENC_AC1_41,
    huffenc_ac1_42: HUFFENC_AC1_42,
    huffenc_ac1_43: HUFFENC_AC1_43,
    huffenc_ac1_44: HUFFENC_AC1_44,
    huffenc_ac1_45: HUFFENC_AC1_45,
    huffenc_ac1_46: HUFFENC_AC1_46,
    huffenc_ac1_47: HUFFENC_AC1_47,
    huffenc_ac1_48: HUFFENC_AC1_48,
    huffenc_ac1_49: HUFFENC_AC1_49,
    huffenc_ac1_50: HUFFENC_AC1_50,
    huffenc_ac1_51: HUFFENC_AC1_51,
    huffenc_ac1_52: HUFFENC_AC1_52,
    huffenc_ac1_53: HUFFENC_AC1_53,
    huffenc_ac1_54: HUFFENC_AC1_54,
    huffenc_ac1_55: HUFFENC_AC1_55,
    huffenc_ac1_56: HUFFENC_AC1_56,
    huffenc_ac1_57: HUFFENC_AC1_57,
    huffenc_ac1_58: HUFFENC_AC1_58,
    huffenc_ac1_59: HUFFENC_AC1_59,
    huffenc_ac1_60: HUFFENC_AC1_60,
    huffenc_ac1_61: HUFFENC_AC1_61,
    huffenc_ac1_62: HUFFENC_AC1_62,
    huffenc_ac1_63: HUFFENC_AC1_63,
    huffenc_ac1_64: HUFFENC_AC1_64,
    huffenc_ac1_65: HUFFENC_AC1_65,
    huffenc_ac1_66: HUFFENC_AC1_66,
    huffenc_ac1_67: HUFFENC_AC1_67,
    huffenc_ac1_68: HUFFENC_AC1_68,
    huffenc_ac1_69: HUFFENC_AC1_69,
    huffenc_ac1_70: HUFFENC_AC1_70,
    huffenc_ac1_71: HUFFENC_AC1_71,
    huffenc_ac1_72: HUFFENC_AC1_72,
    huffenc_ac1_73: HUFFENC_AC1_73,
    huffenc_ac1_74: HUFFENC_AC1_74,
    huffenc_ac1_75: HUFFENC_AC1_75,
    huffenc_ac1_76: HUFFENC_AC1_76,
    huffenc_ac1_77: HUFFENC_AC1_77,
    huffenc_ac1_78: HUFFENC_AC1_78,
    huffenc_ac1_79: HUFFENC_AC1_79,
    huffenc_ac1_80: HUFFENC_AC1_80,
    huffenc_ac1_81: HUFFENC_AC1_81,
    huffenc_ac1_82: HUFFENC_AC1_82,
    huffenc_ac1_83: HUFFENC_AC1_83,
    huffenc_ac1_84: HUFFENC_AC1_84,
    huffenc_ac1_85: HUFFENC_AC1_85,
    huffenc_ac1_86: HUFFENC_AC1_86,
    huffenc_ac1_87: HUFFENC_AC1_87,
    huffenc_dc0_0: HUFFENC_DC0_0,
    huffenc_dc0_1: HUFFENC_DC0_1,
    huffenc_dc0_2: HUFFENC_DC0_2,
    huffenc_dc0_3: HUFFENC_DC0_3,
    huffenc_dc0_4: HUFFENC_DC0_4,
    huffenc_dc0_5: HUFFENC_DC0_5,
    huffenc_dc0_6: HUFFENC_DC0_6,
    huffenc_dc0_7: HUFFENC_DC0_7,
    huffenc_dc1_0: HUFFENC_DC1_0,
    huffenc_dc1_1: HUFFENC_DC1_1,
    huffenc_dc1_2: HUFFENC_DC1_2,
    huffenc_dc1_3: HUFFENC_DC1_3,
    huffenc_dc1_4: HUFFENC_DC1_4,
    huffenc_dc1_5: HUFFENC_DC1_5,
    huffenc_dc1_6: HUFFENC_DC1_6,
    huffenc_dc1_7: HUFFENC_DC1_7,
}
impl RegisterBlock {
    ///0x00 - JPEG codec control register
    #[inline(always)]
    pub const fn confr0(&self) -> &CONFR0 {
        &self.confr0
    }
    ///0x04 - JPEG codec configuration register 1
    #[inline(always)]
    pub const fn confr1(&self) -> &CONFR1 {
        &self.confr1
    }
    ///0x08 - JPEG codec configuration register 2
    #[inline(always)]
    pub const fn confr2(&self) -> &CONFR2 {
        &self.confr2
    }
    ///0x0c - JPEG codec configuration register 3
    #[inline(always)]
    pub const fn confr3(&self) -> &CONFR3 {
        &self.confr3
    }
    ///0x10 - JPEG codec configuration register 4
    #[inline(always)]
    pub const fn confr4(&self) -> &CONFR4 {
        &self.confr4
    }
    ///0x14 - JPEG codec configuration register 5
    #[inline(always)]
    pub const fn confr5(&self) -> &CONFR5 {
        &self.confr5
    }
    ///0x18 - JPEG codec configuration register 6
    #[inline(always)]
    pub const fn confr6(&self) -> &CONFR6 {
        &self.confr6
    }
    ///0x1c - JPEG codec configuration register 7
    #[inline(always)]
    pub const fn confr7(&self) -> &CONFR7 {
        &self.confr7
    }
    ///0x30 - JPEG control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x34 - JPEG status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x38 - JPEG clear flag register
    #[inline(always)]
    pub const fn cfr(&self) -> &CFR {
        &self.cfr
    }
    ///0x40 - JPEG data input register
    #[inline(always)]
    pub const fn dir(&self) -> &DIR {
        &self.dir
    }
    ///0x44 - JPEG data output register
    #[inline(always)]
    pub const fn dor(&self) -> &DOR {
        &self.dor
    }
    ///0x50 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_0(&self) -> &QMEM0_0 {
        &self.qmem0_0
    }
    ///0x54 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_1(&self) -> &QMEM0_1 {
        &self.qmem0_1
    }
    ///0x58 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_2(&self) -> &QMEM0_2 {
        &self.qmem0_2
    }
    ///0x5c - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_3(&self) -> &QMEM0_3 {
        &self.qmem0_3
    }
    ///0x60 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_4(&self) -> &QMEM0_4 {
        &self.qmem0_4
    }
    ///0x64 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_5(&self) -> &QMEM0_5 {
        &self.qmem0_5
    }
    ///0x68 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_6(&self) -> &QMEM0_6 {
        &self.qmem0_6
    }
    ///0x6c - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_7(&self) -> &QMEM0_7 {
        &self.qmem0_7
    }
    ///0x70 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_8(&self) -> &QMEM0_8 {
        &self.qmem0_8
    }
    ///0x74 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_9(&self) -> &QMEM0_9 {
        &self.qmem0_9
    }
    ///0x78 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_10(&self) -> &QMEM0_10 {
        &self.qmem0_10
    }
    ///0x7c - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_11(&self) -> &QMEM0_11 {
        &self.qmem0_11
    }
    ///0x80 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_12(&self) -> &QMEM0_12 {
        &self.qmem0_12
    }
    ///0x84 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_13(&self) -> &QMEM0_13 {
        &self.qmem0_13
    }
    ///0x88 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_14(&self) -> &QMEM0_14 {
        &self.qmem0_14
    }
    ///0x8c - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0_15(&self) -> &QMEM0_15 {
        &self.qmem0_15
    }
    ///0x90 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_0(&self) -> &QMEM1_0 {
        &self.qmem1_0
    }
    ///0x94 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_1(&self) -> &QMEM1_1 {
        &self.qmem1_1
    }
    ///0x98 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_2(&self) -> &QMEM1_2 {
        &self.qmem1_2
    }
    ///0x9c - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_3(&self) -> &QMEM1_3 {
        &self.qmem1_3
    }
    ///0xa0 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_4(&self) -> &QMEM1_4 {
        &self.qmem1_4
    }
    ///0xa4 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_5(&self) -> &QMEM1_5 {
        &self.qmem1_5
    }
    ///0xa8 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_6(&self) -> &QMEM1_6 {
        &self.qmem1_6
    }
    ///0xac - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_7(&self) -> &QMEM1_7 {
        &self.qmem1_7
    }
    ///0xb0 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_8(&self) -> &QMEM1_8 {
        &self.qmem1_8
    }
    ///0xb4 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_9(&self) -> &QMEM1_9 {
        &self.qmem1_9
    }
    ///0xb8 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_10(&self) -> &QMEM1_10 {
        &self.qmem1_10
    }
    ///0xbc - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_11(&self) -> &QMEM1_11 {
        &self.qmem1_11
    }
    ///0xc0 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_12(&self) -> &QMEM1_12 {
        &self.qmem1_12
    }
    ///0xc4 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_13(&self) -> &QMEM1_13 {
        &self.qmem1_13
    }
    ///0xc8 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_14(&self) -> &QMEM1_14 {
        &self.qmem1_14
    }
    ///0xcc - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1_15(&self) -> &QMEM1_15 {
        &self.qmem1_15
    }
    ///0xd0 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_0(&self) -> &QMEM2_0 {
        &self.qmem2_0
    }
    ///0xd4 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_1(&self) -> &QMEM2_1 {
        &self.qmem2_1
    }
    ///0xd8 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_2(&self) -> &QMEM2_2 {
        &self.qmem2_2
    }
    ///0xdc - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_3(&self) -> &QMEM2_3 {
        &self.qmem2_3
    }
    ///0xe0 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_4(&self) -> &QMEM2_4 {
        &self.qmem2_4
    }
    ///0xe4 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_5(&self) -> &QMEM2_5 {
        &self.qmem2_5
    }
    ///0xe8 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_6(&self) -> &QMEM2_6 {
        &self.qmem2_6
    }
    ///0xec - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_7(&self) -> &QMEM2_7 {
        &self.qmem2_7
    }
    ///0xf0 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_8(&self) -> &QMEM2_8 {
        &self.qmem2_8
    }
    ///0xf4 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_9(&self) -> &QMEM2_9 {
        &self.qmem2_9
    }
    ///0xf8 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_10(&self) -> &QMEM2_10 {
        &self.qmem2_10
    }
    ///0xfc - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_11(&self) -> &QMEM2_11 {
        &self.qmem2_11
    }
    ///0x100 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_12(&self) -> &QMEM2_12 {
        &self.qmem2_12
    }
    ///0x104 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_13(&self) -> &QMEM2_13 {
        &self.qmem2_13
    }
    ///0x108 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_14(&self) -> &QMEM2_14 {
        &self.qmem2_14
    }
    ///0x10c - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2_15(&self) -> &QMEM2_15 {
        &self.qmem2_15
    }
    ///0x110 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_0(&self) -> &QMEM3_0 {
        &self.qmem3_0
    }
    ///0x114 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_1(&self) -> &QMEM3_1 {
        &self.qmem3_1
    }
    ///0x118 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_2(&self) -> &QMEM3_2 {
        &self.qmem3_2
    }
    ///0x11c - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_3(&self) -> &QMEM3_3 {
        &self.qmem3_3
    }
    ///0x120 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_4(&self) -> &QMEM3_4 {
        &self.qmem3_4
    }
    ///0x124 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_5(&self) -> &QMEM3_5 {
        &self.qmem3_5
    }
    ///0x128 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_6(&self) -> &QMEM3_6 {
        &self.qmem3_6
    }
    ///0x12c - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_7(&self) -> &QMEM3_7 {
        &self.qmem3_7
    }
    ///0x130 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_8(&self) -> &QMEM3_8 {
        &self.qmem3_8
    }
    ///0x134 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_9(&self) -> &QMEM3_9 {
        &self.qmem3_9
    }
    ///0x138 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_10(&self) -> &QMEM3_10 {
        &self.qmem3_10
    }
    ///0x13c - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_11(&self) -> &QMEM3_11 {
        &self.qmem3_11
    }
    ///0x140 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_12(&self) -> &QMEM3_12 {
        &self.qmem3_12
    }
    ///0x144 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_13(&self) -> &QMEM3_13 {
        &self.qmem3_13
    }
    ///0x148 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_14(&self) -> &QMEM3_14 {
        &self.qmem3_14
    }
    ///0x14c - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3_15(&self) -> &QMEM3_15 {
        &self.qmem3_15
    }
    ///0x150 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin0_0(&self) -> &HUFFMIN0_0 {
        &self.huffmin0_0
    }
    ///0x154 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin0_1(&self) -> &HUFFMIN0_1 {
        &self.huffmin0_1
    }
    ///0x158 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin0_2(&self) -> &HUFFMIN0_2 {
        &self.huffmin0_2
    }
    ///0x15c - JPEG Huffman min 0
    #[inline(always)]
    pub const fn huffmin0_3(&self) -> &HUFFMIN0_3 {
        &self.huffmin0_3
    }
    ///0x160 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin1_0(&self) -> &HUFFMIN1_0 {
        &self.huffmin1_0
    }
    ///0x164 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin1_1(&self) -> &HUFFMIN1_1 {
        &self.huffmin1_1
    }
    ///0x168 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin1_2(&self) -> &HUFFMIN1_2 {
        &self.huffmin1_2
    }
    ///0x16c - JPEG Huffman min 1
    #[inline(always)]
    pub const fn huffmin1_3(&self) -> &HUFFMIN1_3 {
        &self.huffmin1_3
    }
    ///0x170 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin2_0(&self) -> &HUFFMIN2_0 {
        &self.huffmin2_0
    }
    ///0x174 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin2_1(&self) -> &HUFFMIN2_1 {
        &self.huffmin2_1
    }
    ///0x178 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin2_2(&self) -> &HUFFMIN2_2 {
        &self.huffmin2_2
    }
    ///0x17c - JPEG Huffman min 2
    #[inline(always)]
    pub const fn huffmin2_3(&self) -> &HUFFMIN2_3 {
        &self.huffmin2_3
    }
    ///0x180 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin3_0(&self) -> &HUFFMIN3_0 {
        &self.huffmin3_0
    }
    ///0x184 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin3_1(&self) -> &HUFFMIN3_1 {
        &self.huffmin3_1
    }
    ///0x188 - JPEG Huffman min
    #[inline(always)]
    pub const fn huffmin3_2(&self) -> &HUFFMIN3_2 {
        &self.huffmin3_2
    }
    ///0x18c - JPEG Huffman min 3
    #[inline(always)]
    pub const fn huffmin3_3(&self) -> &HUFFMIN3_3 {
        &self.huffmin3_3
    }
    ///0x190 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase0(&self) -> &HUFFBASE0 {
        &self.huffbase0
    }
    ///0x194 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase1(&self) -> &HUFFBASE1 {
        &self.huffbase1
    }
    ///0x198 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase2(&self) -> &HUFFBASE2 {
        &self.huffbase2
    }
    ///0x19c - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase3(&self) -> &HUFFBASE3 {
        &self.huffbase3
    }
    ///0x1a0 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase4(&self) -> &HUFFBASE4 {
        &self.huffbase4
    }
    ///0x1a4 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase5(&self) -> &HUFFBASE5 {
        &self.huffbase5
    }
    ///0x1a8 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase6(&self) -> &HUFFBASE6 {
        &self.huffbase6
    }
    ///0x1ac - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase7(&self) -> &HUFFBASE7 {
        &self.huffbase7
    }
    ///0x1b0 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase8(&self) -> &HUFFBASE8 {
        &self.huffbase8
    }
    ///0x1b4 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase9(&self) -> &HUFFBASE9 {
        &self.huffbase9
    }
    ///0x1b8 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase10(&self) -> &HUFFBASE10 {
        &self.huffbase10
    }
    ///0x1bc - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase11(&self) -> &HUFFBASE11 {
        &self.huffbase11
    }
    ///0x1c0 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase12(&self) -> &HUFFBASE12 {
        &self.huffbase12
    }
    ///0x1c4 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase13(&self) -> &HUFFBASE13 {
        &self.huffbase13
    }
    ///0x1c8 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase14(&self) -> &HUFFBASE14 {
        &self.huffbase14
    }
    ///0x1cc - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase15(&self) -> &HUFFBASE15 {
        &self.huffbase15
    }
    ///0x1d0 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase16(&self) -> &HUFFBASE16 {
        &self.huffbase16
    }
    ///0x1d4 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase17(&self) -> &HUFFBASE17 {
        &self.huffbase17
    }
    ///0x1d8 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase18(&self) -> &HUFFBASE18 {
        &self.huffbase18
    }
    ///0x1dc - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase19(&self) -> &HUFFBASE19 {
        &self.huffbase19
    }
    ///0x1e0 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase20(&self) -> &HUFFBASE20 {
        &self.huffbase20
    }
    ///0x1e4 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase21(&self) -> &HUFFBASE21 {
        &self.huffbase21
    }
    ///0x1e8 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase22(&self) -> &HUFFBASE22 {
        &self.huffbase22
    }
    ///0x1ec - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase23(&self) -> &HUFFBASE23 {
        &self.huffbase23
    }
    ///0x1f0 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase24(&self) -> &HUFFBASE24 {
        &self.huffbase24
    }
    ///0x1f4 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase25(&self) -> &HUFFBASE25 {
        &self.huffbase25
    }
    ///0x1f8 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase26(&self) -> &HUFFBASE26 {
        &self.huffbase26
    }
    ///0x1fc - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase27(&self) -> &HUFFBASE27 {
        &self.huffbase27
    }
    ///0x200 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase28(&self) -> &HUFFBASE28 {
        &self.huffbase28
    }
    ///0x204 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase29(&self) -> &HUFFBASE29 {
        &self.huffbase29
    }
    ///0x208 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase30(&self) -> &HUFFBASE30 {
        &self.huffbase30
    }
    ///0x20c - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase31(&self) -> &HUFFBASE31 {
        &self.huffbase31
    }
    ///0x210 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb0(&self) -> &HUFFSYMB0 {
        &self.huffsymb0
    }
    ///0x214 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb1(&self) -> &HUFFSYMB1 {
        &self.huffsymb1
    }
    ///0x218 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb2(&self) -> &HUFFSYMB2 {
        &self.huffsymb2
    }
    ///0x21c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb3(&self) -> &HUFFSYMB3 {
        &self.huffsymb3
    }
    ///0x220 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb4(&self) -> &HUFFSYMB4 {
        &self.huffsymb4
    }
    ///0x224 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb5(&self) -> &HUFFSYMB5 {
        &self.huffsymb5
    }
    ///0x228 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb6(&self) -> &HUFFSYMB6 {
        &self.huffsymb6
    }
    ///0x22c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb7(&self) -> &HUFFSYMB7 {
        &self.huffsymb7
    }
    ///0x230 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb8(&self) -> &HUFFSYMB8 {
        &self.huffsymb8
    }
    ///0x234 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb9(&self) -> &HUFFSYMB9 {
        &self.huffsymb9
    }
    ///0x238 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb10(&self) -> &HUFFSYMB10 {
        &self.huffsymb10
    }
    ///0x23c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb11(&self) -> &HUFFSYMB11 {
        &self.huffsymb11
    }
    ///0x240 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb12(&self) -> &HUFFSYMB12 {
        &self.huffsymb12
    }
    ///0x244 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb13(&self) -> &HUFFSYMB13 {
        &self.huffsymb13
    }
    ///0x248 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb14(&self) -> &HUFFSYMB14 {
        &self.huffsymb14
    }
    ///0x24c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb15(&self) -> &HUFFSYMB15 {
        &self.huffsymb15
    }
    ///0x250 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb16(&self) -> &HUFFSYMB16 {
        &self.huffsymb16
    }
    ///0x254 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb17(&self) -> &HUFFSYMB17 {
        &self.huffsymb17
    }
    ///0x258 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb18(&self) -> &HUFFSYMB18 {
        &self.huffsymb18
    }
    ///0x25c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb19(&self) -> &HUFFSYMB19 {
        &self.huffsymb19
    }
    ///0x260 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb20(&self) -> &HUFFSYMB20 {
        &self.huffsymb20
    }
    ///0x264 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb21(&self) -> &HUFFSYMB21 {
        &self.huffsymb21
    }
    ///0x268 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb22(&self) -> &HUFFSYMB22 {
        &self.huffsymb22
    }
    ///0x26c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb23(&self) -> &HUFFSYMB23 {
        &self.huffsymb23
    }
    ///0x270 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb24(&self) -> &HUFFSYMB24 {
        &self.huffsymb24
    }
    ///0x274 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb25(&self) -> &HUFFSYMB25 {
        &self.huffsymb25
    }
    ///0x278 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb26(&self) -> &HUFFSYMB26 {
        &self.huffsymb26
    }
    ///0x27c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb27(&self) -> &HUFFSYMB27 {
        &self.huffsymb27
    }
    ///0x280 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb28(&self) -> &HUFFSYMB28 {
        &self.huffsymb28
    }
    ///0x284 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb29(&self) -> &HUFFSYMB29 {
        &self.huffsymb29
    }
    ///0x288 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb30(&self) -> &HUFFSYMB30 {
        &self.huffsymb30
    }
    ///0x28c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb31(&self) -> &HUFFSYMB31 {
        &self.huffsymb31
    }
    ///0x290 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb32(&self) -> &HUFFSYMB32 {
        &self.huffsymb32
    }
    ///0x294 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb33(&self) -> &HUFFSYMB33 {
        &self.huffsymb33
    }
    ///0x298 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb34(&self) -> &HUFFSYMB34 {
        &self.huffsymb34
    }
    ///0x29c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb35(&self) -> &HUFFSYMB35 {
        &self.huffsymb35
    }
    ///0x2a0 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb36(&self) -> &HUFFSYMB36 {
        &self.huffsymb36
    }
    ///0x2a4 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb37(&self) -> &HUFFSYMB37 {
        &self.huffsymb37
    }
    ///0x2a8 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb38(&self) -> &HUFFSYMB38 {
        &self.huffsymb38
    }
    ///0x2ac - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb39(&self) -> &HUFFSYMB39 {
        &self.huffsymb39
    }
    ///0x2b0 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb40(&self) -> &HUFFSYMB40 {
        &self.huffsymb40
    }
    ///0x2b4 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb41(&self) -> &HUFFSYMB41 {
        &self.huffsymb41
    }
    ///0x2b8 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb42(&self) -> &HUFFSYMB42 {
        &self.huffsymb42
    }
    ///0x2bc - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb43(&self) -> &HUFFSYMB43 {
        &self.huffsymb43
    }
    ///0x2c0 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb44(&self) -> &HUFFSYMB44 {
        &self.huffsymb44
    }
    ///0x2c4 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb45(&self) -> &HUFFSYMB45 {
        &self.huffsymb45
    }
    ///0x2c8 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb46(&self) -> &HUFFSYMB46 {
        &self.huffsymb46
    }
    ///0x2cc - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb47(&self) -> &HUFFSYMB47 {
        &self.huffsymb47
    }
    ///0x2d0 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb48(&self) -> &HUFFSYMB48 {
        &self.huffsymb48
    }
    ///0x2d4 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb49(&self) -> &HUFFSYMB49 {
        &self.huffsymb49
    }
    ///0x2d8 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb50(&self) -> &HUFFSYMB50 {
        &self.huffsymb50
    }
    ///0x2dc - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb51(&self) -> &HUFFSYMB51 {
        &self.huffsymb51
    }
    ///0x2e0 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb52(&self) -> &HUFFSYMB52 {
        &self.huffsymb52
    }
    ///0x2e4 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb53(&self) -> &HUFFSYMB53 {
        &self.huffsymb53
    }
    ///0x2e8 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb54(&self) -> &HUFFSYMB54 {
        &self.huffsymb54
    }
    ///0x2ec - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb55(&self) -> &HUFFSYMB55 {
        &self.huffsymb55
    }
    ///0x2f0 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb56(&self) -> &HUFFSYMB56 {
        &self.huffsymb56
    }
    ///0x2f4 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb57(&self) -> &HUFFSYMB57 {
        &self.huffsymb57
    }
    ///0x2f8 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb58(&self) -> &HUFFSYMB58 {
        &self.huffsymb58
    }
    ///0x2fc - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb59(&self) -> &HUFFSYMB59 {
        &self.huffsymb59
    }
    ///0x300 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb60(&self) -> &HUFFSYMB60 {
        &self.huffsymb60
    }
    ///0x304 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb61(&self) -> &HUFFSYMB61 {
        &self.huffsymb61
    }
    ///0x308 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb62(&self) -> &HUFFSYMB62 {
        &self.huffsymb62
    }
    ///0x30c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb63(&self) -> &HUFFSYMB63 {
        &self.huffsymb63
    }
    ///0x310 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb64(&self) -> &HUFFSYMB64 {
        &self.huffsymb64
    }
    ///0x314 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb65(&self) -> &HUFFSYMB65 {
        &self.huffsymb65
    }
    ///0x318 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb66(&self) -> &HUFFSYMB66 {
        &self.huffsymb66
    }
    ///0x31c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb67(&self) -> &HUFFSYMB67 {
        &self.huffsymb67
    }
    ///0x320 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb68(&self) -> &HUFFSYMB68 {
        &self.huffsymb68
    }
    ///0x324 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb69(&self) -> &HUFFSYMB69 {
        &self.huffsymb69
    }
    ///0x328 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb70(&self) -> &HUFFSYMB70 {
        &self.huffsymb70
    }
    ///0x32c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb71(&self) -> &HUFFSYMB71 {
        &self.huffsymb71
    }
    ///0x330 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb72(&self) -> &HUFFSYMB72 {
        &self.huffsymb72
    }
    ///0x334 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb73(&self) -> &HUFFSYMB73 {
        &self.huffsymb73
    }
    ///0x338 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb74(&self) -> &HUFFSYMB74 {
        &self.huffsymb74
    }
    ///0x33c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb75(&self) -> &HUFFSYMB75 {
        &self.huffsymb75
    }
    ///0x340 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb76(&self) -> &HUFFSYMB76 {
        &self.huffsymb76
    }
    ///0x344 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb77(&self) -> &HUFFSYMB77 {
        &self.huffsymb77
    }
    ///0x348 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb78(&self) -> &HUFFSYMB78 {
        &self.huffsymb78
    }
    ///0x34c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb79(&self) -> &HUFFSYMB79 {
        &self.huffsymb79
    }
    ///0x350 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb80(&self) -> &HUFFSYMB80 {
        &self.huffsymb80
    }
    ///0x354 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb81(&self) -> &HUFFSYMB81 {
        &self.huffsymb81
    }
    ///0x358 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb82(&self) -> &HUFFSYMB82 {
        &self.huffsymb82
    }
    ///0x35c - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb83(&self) -> &HUFFSYMB83 {
        &self.huffsymb83
    }
    ///0x360 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem0(&self) -> &DHTMEM0 {
        &self.dhtmem0
    }
    ///0x364 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem1(&self) -> &DHTMEM1 {
        &self.dhtmem1
    }
    ///0x368 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem2(&self) -> &DHTMEM2 {
        &self.dhtmem2
    }
    ///0x36c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem3(&self) -> &DHTMEM3 {
        &self.dhtmem3
    }
    ///0x370 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem4(&self) -> &DHTMEM4 {
        &self.dhtmem4
    }
    ///0x374 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem5(&self) -> &DHTMEM5 {
        &self.dhtmem5
    }
    ///0x378 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem6(&self) -> &DHTMEM6 {
        &self.dhtmem6
    }
    ///0x37c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem7(&self) -> &DHTMEM7 {
        &self.dhtmem7
    }
    ///0x380 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem8(&self) -> &DHTMEM8 {
        &self.dhtmem8
    }
    ///0x384 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem9(&self) -> &DHTMEM9 {
        &self.dhtmem9
    }
    ///0x388 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem10(&self) -> &DHTMEM10 {
        &self.dhtmem10
    }
    ///0x38c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem11(&self) -> &DHTMEM11 {
        &self.dhtmem11
    }
    ///0x390 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem12(&self) -> &DHTMEM12 {
        &self.dhtmem12
    }
    ///0x394 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem13(&self) -> &DHTMEM13 {
        &self.dhtmem13
    }
    ///0x398 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem14(&self) -> &DHTMEM14 {
        &self.dhtmem14
    }
    ///0x39c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem15(&self) -> &DHTMEM15 {
        &self.dhtmem15
    }
    ///0x3a0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem16(&self) -> &DHTMEM16 {
        &self.dhtmem16
    }
    ///0x3a4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem17(&self) -> &DHTMEM17 {
        &self.dhtmem17
    }
    ///0x3a8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem18(&self) -> &DHTMEM18 {
        &self.dhtmem18
    }
    ///0x3ac - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem19(&self) -> &DHTMEM19 {
        &self.dhtmem19
    }
    ///0x3b0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem20(&self) -> &DHTMEM20 {
        &self.dhtmem20
    }
    ///0x3b4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem21(&self) -> &DHTMEM21 {
        &self.dhtmem21
    }
    ///0x3b8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem22(&self) -> &DHTMEM22 {
        &self.dhtmem22
    }
    ///0x3bc - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem23(&self) -> &DHTMEM23 {
        &self.dhtmem23
    }
    ///0x3c0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem24(&self) -> &DHTMEM24 {
        &self.dhtmem24
    }
    ///0x3c4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem25(&self) -> &DHTMEM25 {
        &self.dhtmem25
    }
    ///0x3c8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem26(&self) -> &DHTMEM26 {
        &self.dhtmem26
    }
    ///0x3cc - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem27(&self) -> &DHTMEM27 {
        &self.dhtmem27
    }
    ///0x3d0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem28(&self) -> &DHTMEM28 {
        &self.dhtmem28
    }
    ///0x3d4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem29(&self) -> &DHTMEM29 {
        &self.dhtmem29
    }
    ///0x3d8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem30(&self) -> &DHTMEM30 {
        &self.dhtmem30
    }
    ///0x3dc - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem31(&self) -> &DHTMEM31 {
        &self.dhtmem31
    }
    ///0x3e0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem32(&self) -> &DHTMEM32 {
        &self.dhtmem32
    }
    ///0x3e4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem33(&self) -> &DHTMEM33 {
        &self.dhtmem33
    }
    ///0x3e8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem34(&self) -> &DHTMEM34 {
        &self.dhtmem34
    }
    ///0x3ec - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem35(&self) -> &DHTMEM35 {
        &self.dhtmem35
    }
    ///0x3f0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem36(&self) -> &DHTMEM36 {
        &self.dhtmem36
    }
    ///0x3f4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem37(&self) -> &DHTMEM37 {
        &self.dhtmem37
    }
    ///0x3f8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem38(&self) -> &DHTMEM38 {
        &self.dhtmem38
    }
    ///0x3fc - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem39(&self) -> &DHTMEM39 {
        &self.dhtmem39
    }
    ///0x400 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem40(&self) -> &DHTMEM40 {
        &self.dhtmem40
    }
    ///0x404 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem41(&self) -> &DHTMEM41 {
        &self.dhtmem41
    }
    ///0x408 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem42(&self) -> &DHTMEM42 {
        &self.dhtmem42
    }
    ///0x40c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem43(&self) -> &DHTMEM43 {
        &self.dhtmem43
    }
    ///0x410 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem44(&self) -> &DHTMEM44 {
        &self.dhtmem44
    }
    ///0x414 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem45(&self) -> &DHTMEM45 {
        &self.dhtmem45
    }
    ///0x418 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem46(&self) -> &DHTMEM46 {
        &self.dhtmem46
    }
    ///0x41c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem47(&self) -> &DHTMEM47 {
        &self.dhtmem47
    }
    ///0x420 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem48(&self) -> &DHTMEM48 {
        &self.dhtmem48
    }
    ///0x424 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem49(&self) -> &DHTMEM49 {
        &self.dhtmem49
    }
    ///0x428 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem50(&self) -> &DHTMEM50 {
        &self.dhtmem50
    }
    ///0x42c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem51(&self) -> &DHTMEM51 {
        &self.dhtmem51
    }
    ///0x430 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem52(&self) -> &DHTMEM52 {
        &self.dhtmem52
    }
    ///0x434 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem53(&self) -> &DHTMEM53 {
        &self.dhtmem53
    }
    ///0x438 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem54(&self) -> &DHTMEM54 {
        &self.dhtmem54
    }
    ///0x43c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem55(&self) -> &DHTMEM55 {
        &self.dhtmem55
    }
    ///0x440 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem56(&self) -> &DHTMEM56 {
        &self.dhtmem56
    }
    ///0x444 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem57(&self) -> &DHTMEM57 {
        &self.dhtmem57
    }
    ///0x448 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem58(&self) -> &DHTMEM58 {
        &self.dhtmem58
    }
    ///0x44c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem59(&self) -> &DHTMEM59 {
        &self.dhtmem59
    }
    ///0x450 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem60(&self) -> &DHTMEM60 {
        &self.dhtmem60
    }
    ///0x454 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem61(&self) -> &DHTMEM61 {
        &self.dhtmem61
    }
    ///0x458 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem62(&self) -> &DHTMEM62 {
        &self.dhtmem62
    }
    ///0x45c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem63(&self) -> &DHTMEM63 {
        &self.dhtmem63
    }
    ///0x460 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem64(&self) -> &DHTMEM64 {
        &self.dhtmem64
    }
    ///0x464 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem65(&self) -> &DHTMEM65 {
        &self.dhtmem65
    }
    ///0x468 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem66(&self) -> &DHTMEM66 {
        &self.dhtmem66
    }
    ///0x46c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem67(&self) -> &DHTMEM67 {
        &self.dhtmem67
    }
    ///0x470 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem68(&self) -> &DHTMEM68 {
        &self.dhtmem68
    }
    ///0x474 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem69(&self) -> &DHTMEM69 {
        &self.dhtmem69
    }
    ///0x478 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem70(&self) -> &DHTMEM70 {
        &self.dhtmem70
    }
    ///0x47c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem71(&self) -> &DHTMEM71 {
        &self.dhtmem71
    }
    ///0x480 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem72(&self) -> &DHTMEM72 {
        &self.dhtmem72
    }
    ///0x484 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem73(&self) -> &DHTMEM73 {
        &self.dhtmem73
    }
    ///0x488 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem74(&self) -> &DHTMEM74 {
        &self.dhtmem74
    }
    ///0x48c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem75(&self) -> &DHTMEM75 {
        &self.dhtmem75
    }
    ///0x490 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem76(&self) -> &DHTMEM76 {
        &self.dhtmem76
    }
    ///0x494 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem77(&self) -> &DHTMEM77 {
        &self.dhtmem77
    }
    ///0x498 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem78(&self) -> &DHTMEM78 {
        &self.dhtmem78
    }
    ///0x49c - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem79(&self) -> &DHTMEM79 {
        &self.dhtmem79
    }
    ///0x4a0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem80(&self) -> &DHTMEM80 {
        &self.dhtmem80
    }
    ///0x4a4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem81(&self) -> &DHTMEM81 {
        &self.dhtmem81
    }
    ///0x4a8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem82(&self) -> &DHTMEM82 {
        &self.dhtmem82
    }
    ///0x4ac - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem83(&self) -> &DHTMEM83 {
        &self.dhtmem83
    }
    ///0x4b0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem84(&self) -> &DHTMEM84 {
        &self.dhtmem84
    }
    ///0x4b4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem85(&self) -> &DHTMEM85 {
        &self.dhtmem85
    }
    ///0x4b8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem86(&self) -> &DHTMEM86 {
        &self.dhtmem86
    }
    ///0x4bc - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem87(&self) -> &DHTMEM87 {
        &self.dhtmem87
    }
    ///0x4c0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem88(&self) -> &DHTMEM88 {
        &self.dhtmem88
    }
    ///0x4c4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem89(&self) -> &DHTMEM89 {
        &self.dhtmem89
    }
    ///0x4c8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem90(&self) -> &DHTMEM90 {
        &self.dhtmem90
    }
    ///0x4cc - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem91(&self) -> &DHTMEM91 {
        &self.dhtmem91
    }
    ///0x4d0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem92(&self) -> &DHTMEM92 {
        &self.dhtmem92
    }
    ///0x4d4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem93(&self) -> &DHTMEM93 {
        &self.dhtmem93
    }
    ///0x4d8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem94(&self) -> &DHTMEM94 {
        &self.dhtmem94
    }
    ///0x4dc - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem95(&self) -> &DHTMEM95 {
        &self.dhtmem95
    }
    ///0x4e0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem96(&self) -> &DHTMEM96 {
        &self.dhtmem96
    }
    ///0x4e4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem97(&self) -> &DHTMEM97 {
        &self.dhtmem97
    }
    ///0x4e8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem98(&self) -> &DHTMEM98 {
        &self.dhtmem98
    }
    ///0x4ec - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem99(&self) -> &DHTMEM99 {
        &self.dhtmem99
    }
    ///0x4f0 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem100(&self) -> &DHTMEM100 {
        &self.dhtmem100
    }
    ///0x4f4 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem101(&self) -> &DHTMEM101 {
        &self.dhtmem101
    }
    ///0x4f8 - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem102(&self) -> &DHTMEM102 {
        &self.dhtmem102
    }
    ///0x500 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_0(&self) -> &HUFFENC_AC0_0 {
        &self.huffenc_ac0_0
    }
    ///0x504 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_1(&self) -> &HUFFENC_AC0_1 {
        &self.huffenc_ac0_1
    }
    ///0x508 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_2(&self) -> &HUFFENC_AC0_2 {
        &self.huffenc_ac0_2
    }
    ///0x50c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_3(&self) -> &HUFFENC_AC0_3 {
        &self.huffenc_ac0_3
    }
    ///0x510 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_4(&self) -> &HUFFENC_AC0_4 {
        &self.huffenc_ac0_4
    }
    ///0x514 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_5(&self) -> &HUFFENC_AC0_5 {
        &self.huffenc_ac0_5
    }
    ///0x518 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_6(&self) -> &HUFFENC_AC0_6 {
        &self.huffenc_ac0_6
    }
    ///0x51c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_7(&self) -> &HUFFENC_AC0_7 {
        &self.huffenc_ac0_7
    }
    ///0x520 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_8(&self) -> &HUFFENC_AC0_8 {
        &self.huffenc_ac0_8
    }
    ///0x524 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_9(&self) -> &HUFFENC_AC0_9 {
        &self.huffenc_ac0_9
    }
    ///0x528 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_10(&self) -> &HUFFENC_AC0_10 {
        &self.huffenc_ac0_10
    }
    ///0x52c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_11(&self) -> &HUFFENC_AC0_11 {
        &self.huffenc_ac0_11
    }
    ///0x530 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_12(&self) -> &HUFFENC_AC0_12 {
        &self.huffenc_ac0_12
    }
    ///0x534 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_13(&self) -> &HUFFENC_AC0_13 {
        &self.huffenc_ac0_13
    }
    ///0x538 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_14(&self) -> &HUFFENC_AC0_14 {
        &self.huffenc_ac0_14
    }
    ///0x53c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_15(&self) -> &HUFFENC_AC0_15 {
        &self.huffenc_ac0_15
    }
    ///0x540 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_16(&self) -> &HUFFENC_AC0_16 {
        &self.huffenc_ac0_16
    }
    ///0x544 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_17(&self) -> &HUFFENC_AC0_17 {
        &self.huffenc_ac0_17
    }
    ///0x548 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_18(&self) -> &HUFFENC_AC0_18 {
        &self.huffenc_ac0_18
    }
    ///0x54c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_19(&self) -> &HUFFENC_AC0_19 {
        &self.huffenc_ac0_19
    }
    ///0x550 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_20(&self) -> &HUFFENC_AC0_20 {
        &self.huffenc_ac0_20
    }
    ///0x554 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_21(&self) -> &HUFFENC_AC0_21 {
        &self.huffenc_ac0_21
    }
    ///0x558 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_22(&self) -> &HUFFENC_AC0_22 {
        &self.huffenc_ac0_22
    }
    ///0x55c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_23(&self) -> &HUFFENC_AC0_23 {
        &self.huffenc_ac0_23
    }
    ///0x560 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_24(&self) -> &HUFFENC_AC0_24 {
        &self.huffenc_ac0_24
    }
    ///0x564 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_25(&self) -> &HUFFENC_AC0_25 {
        &self.huffenc_ac0_25
    }
    ///0x568 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_26(&self) -> &HUFFENC_AC0_26 {
        &self.huffenc_ac0_26
    }
    ///0x56c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_27(&self) -> &HUFFENC_AC0_27 {
        &self.huffenc_ac0_27
    }
    ///0x570 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_28(&self) -> &HUFFENC_AC0_28 {
        &self.huffenc_ac0_28
    }
    ///0x574 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_29(&self) -> &HUFFENC_AC0_29 {
        &self.huffenc_ac0_29
    }
    ///0x578 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_30(&self) -> &HUFFENC_AC0_30 {
        &self.huffenc_ac0_30
    }
    ///0x57c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_31(&self) -> &HUFFENC_AC0_31 {
        &self.huffenc_ac0_31
    }
    ///0x580 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_32(&self) -> &HUFFENC_AC0_32 {
        &self.huffenc_ac0_32
    }
    ///0x584 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_33(&self) -> &HUFFENC_AC0_33 {
        &self.huffenc_ac0_33
    }
    ///0x588 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_34(&self) -> &HUFFENC_AC0_34 {
        &self.huffenc_ac0_34
    }
    ///0x58c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_35(&self) -> &HUFFENC_AC0_35 {
        &self.huffenc_ac0_35
    }
    ///0x590 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_36(&self) -> &HUFFENC_AC0_36 {
        &self.huffenc_ac0_36
    }
    ///0x594 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_37(&self) -> &HUFFENC_AC0_37 {
        &self.huffenc_ac0_37
    }
    ///0x598 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_38(&self) -> &HUFFENC_AC0_38 {
        &self.huffenc_ac0_38
    }
    ///0x59c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_39(&self) -> &HUFFENC_AC0_39 {
        &self.huffenc_ac0_39
    }
    ///0x5a0 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_40(&self) -> &HUFFENC_AC0_40 {
        &self.huffenc_ac0_40
    }
    ///0x5a4 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_41(&self) -> &HUFFENC_AC0_41 {
        &self.huffenc_ac0_41
    }
    ///0x5a8 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_42(&self) -> &HUFFENC_AC0_42 {
        &self.huffenc_ac0_42
    }
    ///0x5ac - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_43(&self) -> &HUFFENC_AC0_43 {
        &self.huffenc_ac0_43
    }
    ///0x5b0 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_44(&self) -> &HUFFENC_AC0_44 {
        &self.huffenc_ac0_44
    }
    ///0x5b4 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_45(&self) -> &HUFFENC_AC0_45 {
        &self.huffenc_ac0_45
    }
    ///0x5b8 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_46(&self) -> &HUFFENC_AC0_46 {
        &self.huffenc_ac0_46
    }
    ///0x5bc - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_47(&self) -> &HUFFENC_AC0_47 {
        &self.huffenc_ac0_47
    }
    ///0x5c0 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_48(&self) -> &HUFFENC_AC0_48 {
        &self.huffenc_ac0_48
    }
    ///0x5c4 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_49(&self) -> &HUFFENC_AC0_49 {
        &self.huffenc_ac0_49
    }
    ///0x5c8 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_50(&self) -> &HUFFENC_AC0_50 {
        &self.huffenc_ac0_50
    }
    ///0x5cc - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_51(&self) -> &HUFFENC_AC0_51 {
        &self.huffenc_ac0_51
    }
    ///0x5d0 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_52(&self) -> &HUFFENC_AC0_52 {
        &self.huffenc_ac0_52
    }
    ///0x5d4 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_53(&self) -> &HUFFENC_AC0_53 {
        &self.huffenc_ac0_53
    }
    ///0x5d8 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_54(&self) -> &HUFFENC_AC0_54 {
        &self.huffenc_ac0_54
    }
    ///0x5dc - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_55(&self) -> &HUFFENC_AC0_55 {
        &self.huffenc_ac0_55
    }
    ///0x5e0 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_56(&self) -> &HUFFENC_AC0_56 {
        &self.huffenc_ac0_56
    }
    ///0x5e4 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_57(&self) -> &HUFFENC_AC0_57 {
        &self.huffenc_ac0_57
    }
    ///0x5e8 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_58(&self) -> &HUFFENC_AC0_58 {
        &self.huffenc_ac0_58
    }
    ///0x5ec - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_59(&self) -> &HUFFENC_AC0_59 {
        &self.huffenc_ac0_59
    }
    ///0x5f0 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_60(&self) -> &HUFFENC_AC0_60 {
        &self.huffenc_ac0_60
    }
    ///0x5f4 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_61(&self) -> &HUFFENC_AC0_61 {
        &self.huffenc_ac0_61
    }
    ///0x5f8 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_62(&self) -> &HUFFENC_AC0_62 {
        &self.huffenc_ac0_62
    }
    ///0x5fc - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_63(&self) -> &HUFFENC_AC0_63 {
        &self.huffenc_ac0_63
    }
    ///0x600 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_64(&self) -> &HUFFENC_AC0_64 {
        &self.huffenc_ac0_64
    }
    ///0x604 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_65(&self) -> &HUFFENC_AC0_65 {
        &self.huffenc_ac0_65
    }
    ///0x608 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_66(&self) -> &HUFFENC_AC0_66 {
        &self.huffenc_ac0_66
    }
    ///0x60c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_67(&self) -> &HUFFENC_AC0_67 {
        &self.huffenc_ac0_67
    }
    ///0x610 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_68(&self) -> &HUFFENC_AC0_68 {
        &self.huffenc_ac0_68
    }
    ///0x614 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_69(&self) -> &HUFFENC_AC0_69 {
        &self.huffenc_ac0_69
    }
    ///0x618 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_70(&self) -> &HUFFENC_AC0_70 {
        &self.huffenc_ac0_70
    }
    ///0x61c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_71(&self) -> &HUFFENC_AC0_71 {
        &self.huffenc_ac0_71
    }
    ///0x620 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_72(&self) -> &HUFFENC_AC0_72 {
        &self.huffenc_ac0_72
    }
    ///0x624 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_73(&self) -> &HUFFENC_AC0_73 {
        &self.huffenc_ac0_73
    }
    ///0x628 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_74(&self) -> &HUFFENC_AC0_74 {
        &self.huffenc_ac0_74
    }
    ///0x62c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_75(&self) -> &HUFFENC_AC0_75 {
        &self.huffenc_ac0_75
    }
    ///0x630 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_76(&self) -> &HUFFENC_AC0_76 {
        &self.huffenc_ac0_76
    }
    ///0x634 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_77(&self) -> &HUFFENC_AC0_77 {
        &self.huffenc_ac0_77
    }
    ///0x638 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_78(&self) -> &HUFFENC_AC0_78 {
        &self.huffenc_ac0_78
    }
    ///0x63c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_79(&self) -> &HUFFENC_AC0_79 {
        &self.huffenc_ac0_79
    }
    ///0x640 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_80(&self) -> &HUFFENC_AC0_80 {
        &self.huffenc_ac0_80
    }
    ///0x644 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_81(&self) -> &HUFFENC_AC0_81 {
        &self.huffenc_ac0_81
    }
    ///0x648 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_82(&self) -> &HUFFENC_AC0_82 {
        &self.huffenc_ac0_82
    }
    ///0x64c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_83(&self) -> &HUFFENC_AC0_83 {
        &self.huffenc_ac0_83
    }
    ///0x650 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_84(&self) -> &HUFFENC_AC0_84 {
        &self.huffenc_ac0_84
    }
    ///0x654 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_85(&self) -> &HUFFENC_AC0_85 {
        &self.huffenc_ac0_85
    }
    ///0x658 - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_86(&self) -> &HUFFENC_AC0_86 {
        &self.huffenc_ac0_86
    }
    ///0x65c - JPEG Huffman encoder AC0
    #[inline(always)]
    pub const fn huffenc_ac0_87(&self) -> &HUFFENC_AC0_87 {
        &self.huffenc_ac0_87
    }
    ///0x660 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_0(&self) -> &HUFFENC_AC1_0 {
        &self.huffenc_ac1_0
    }
    ///0x664 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_1(&self) -> &HUFFENC_AC1_1 {
        &self.huffenc_ac1_1
    }
    ///0x668 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_2(&self) -> &HUFFENC_AC1_2 {
        &self.huffenc_ac1_2
    }
    ///0x66c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_3(&self) -> &HUFFENC_AC1_3 {
        &self.huffenc_ac1_3
    }
    ///0x670 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_4(&self) -> &HUFFENC_AC1_4 {
        &self.huffenc_ac1_4
    }
    ///0x674 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_5(&self) -> &HUFFENC_AC1_5 {
        &self.huffenc_ac1_5
    }
    ///0x678 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_6(&self) -> &HUFFENC_AC1_6 {
        &self.huffenc_ac1_6
    }
    ///0x67c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_7(&self) -> &HUFFENC_AC1_7 {
        &self.huffenc_ac1_7
    }
    ///0x680 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_8(&self) -> &HUFFENC_AC1_8 {
        &self.huffenc_ac1_8
    }
    ///0x684 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_9(&self) -> &HUFFENC_AC1_9 {
        &self.huffenc_ac1_9
    }
    ///0x688 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_10(&self) -> &HUFFENC_AC1_10 {
        &self.huffenc_ac1_10
    }
    ///0x68c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_11(&self) -> &HUFFENC_AC1_11 {
        &self.huffenc_ac1_11
    }
    ///0x690 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_12(&self) -> &HUFFENC_AC1_12 {
        &self.huffenc_ac1_12
    }
    ///0x694 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_13(&self) -> &HUFFENC_AC1_13 {
        &self.huffenc_ac1_13
    }
    ///0x698 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_14(&self) -> &HUFFENC_AC1_14 {
        &self.huffenc_ac1_14
    }
    ///0x69c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_15(&self) -> &HUFFENC_AC1_15 {
        &self.huffenc_ac1_15
    }
    ///0x6a0 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_16(&self) -> &HUFFENC_AC1_16 {
        &self.huffenc_ac1_16
    }
    ///0x6a4 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_17(&self) -> &HUFFENC_AC1_17 {
        &self.huffenc_ac1_17
    }
    ///0x6a8 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_18(&self) -> &HUFFENC_AC1_18 {
        &self.huffenc_ac1_18
    }
    ///0x6ac - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_19(&self) -> &HUFFENC_AC1_19 {
        &self.huffenc_ac1_19
    }
    ///0x6b0 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_20(&self) -> &HUFFENC_AC1_20 {
        &self.huffenc_ac1_20
    }
    ///0x6b4 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_21(&self) -> &HUFFENC_AC1_21 {
        &self.huffenc_ac1_21
    }
    ///0x6b8 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_22(&self) -> &HUFFENC_AC1_22 {
        &self.huffenc_ac1_22
    }
    ///0x6bc - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_23(&self) -> &HUFFENC_AC1_23 {
        &self.huffenc_ac1_23
    }
    ///0x6c0 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_24(&self) -> &HUFFENC_AC1_24 {
        &self.huffenc_ac1_24
    }
    ///0x6c4 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_25(&self) -> &HUFFENC_AC1_25 {
        &self.huffenc_ac1_25
    }
    ///0x6c8 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_26(&self) -> &HUFFENC_AC1_26 {
        &self.huffenc_ac1_26
    }
    ///0x6cc - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_27(&self) -> &HUFFENC_AC1_27 {
        &self.huffenc_ac1_27
    }
    ///0x6d0 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_28(&self) -> &HUFFENC_AC1_28 {
        &self.huffenc_ac1_28
    }
    ///0x6d4 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_29(&self) -> &HUFFENC_AC1_29 {
        &self.huffenc_ac1_29
    }
    ///0x6d8 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_30(&self) -> &HUFFENC_AC1_30 {
        &self.huffenc_ac1_30
    }
    ///0x6dc - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_31(&self) -> &HUFFENC_AC1_31 {
        &self.huffenc_ac1_31
    }
    ///0x6e0 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_32(&self) -> &HUFFENC_AC1_32 {
        &self.huffenc_ac1_32
    }
    ///0x6e4 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_33(&self) -> &HUFFENC_AC1_33 {
        &self.huffenc_ac1_33
    }
    ///0x6e8 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_34(&self) -> &HUFFENC_AC1_34 {
        &self.huffenc_ac1_34
    }
    ///0x6ec - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_35(&self) -> &HUFFENC_AC1_35 {
        &self.huffenc_ac1_35
    }
    ///0x6f0 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_36(&self) -> &HUFFENC_AC1_36 {
        &self.huffenc_ac1_36
    }
    ///0x6f4 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_37(&self) -> &HUFFENC_AC1_37 {
        &self.huffenc_ac1_37
    }
    ///0x6f8 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_38(&self) -> &HUFFENC_AC1_38 {
        &self.huffenc_ac1_38
    }
    ///0x6fc - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_39(&self) -> &HUFFENC_AC1_39 {
        &self.huffenc_ac1_39
    }
    ///0x700 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_40(&self) -> &HUFFENC_AC1_40 {
        &self.huffenc_ac1_40
    }
    ///0x704 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_41(&self) -> &HUFFENC_AC1_41 {
        &self.huffenc_ac1_41
    }
    ///0x708 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_42(&self) -> &HUFFENC_AC1_42 {
        &self.huffenc_ac1_42
    }
    ///0x70c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_43(&self) -> &HUFFENC_AC1_43 {
        &self.huffenc_ac1_43
    }
    ///0x710 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_44(&self) -> &HUFFENC_AC1_44 {
        &self.huffenc_ac1_44
    }
    ///0x714 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_45(&self) -> &HUFFENC_AC1_45 {
        &self.huffenc_ac1_45
    }
    ///0x718 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_46(&self) -> &HUFFENC_AC1_46 {
        &self.huffenc_ac1_46
    }
    ///0x71c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_47(&self) -> &HUFFENC_AC1_47 {
        &self.huffenc_ac1_47
    }
    ///0x720 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_48(&self) -> &HUFFENC_AC1_48 {
        &self.huffenc_ac1_48
    }
    ///0x724 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_49(&self) -> &HUFFENC_AC1_49 {
        &self.huffenc_ac1_49
    }
    ///0x728 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_50(&self) -> &HUFFENC_AC1_50 {
        &self.huffenc_ac1_50
    }
    ///0x72c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_51(&self) -> &HUFFENC_AC1_51 {
        &self.huffenc_ac1_51
    }
    ///0x730 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_52(&self) -> &HUFFENC_AC1_52 {
        &self.huffenc_ac1_52
    }
    ///0x734 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_53(&self) -> &HUFFENC_AC1_53 {
        &self.huffenc_ac1_53
    }
    ///0x738 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_54(&self) -> &HUFFENC_AC1_54 {
        &self.huffenc_ac1_54
    }
    ///0x73c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_55(&self) -> &HUFFENC_AC1_55 {
        &self.huffenc_ac1_55
    }
    ///0x740 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_56(&self) -> &HUFFENC_AC1_56 {
        &self.huffenc_ac1_56
    }
    ///0x744 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_57(&self) -> &HUFFENC_AC1_57 {
        &self.huffenc_ac1_57
    }
    ///0x748 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_58(&self) -> &HUFFENC_AC1_58 {
        &self.huffenc_ac1_58
    }
    ///0x74c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_59(&self) -> &HUFFENC_AC1_59 {
        &self.huffenc_ac1_59
    }
    ///0x750 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_60(&self) -> &HUFFENC_AC1_60 {
        &self.huffenc_ac1_60
    }
    ///0x754 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_61(&self) -> &HUFFENC_AC1_61 {
        &self.huffenc_ac1_61
    }
    ///0x758 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_62(&self) -> &HUFFENC_AC1_62 {
        &self.huffenc_ac1_62
    }
    ///0x75c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_63(&self) -> &HUFFENC_AC1_63 {
        &self.huffenc_ac1_63
    }
    ///0x760 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_64(&self) -> &HUFFENC_AC1_64 {
        &self.huffenc_ac1_64
    }
    ///0x764 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_65(&self) -> &HUFFENC_AC1_65 {
        &self.huffenc_ac1_65
    }
    ///0x768 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_66(&self) -> &HUFFENC_AC1_66 {
        &self.huffenc_ac1_66
    }
    ///0x76c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_67(&self) -> &HUFFENC_AC1_67 {
        &self.huffenc_ac1_67
    }
    ///0x770 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_68(&self) -> &HUFFENC_AC1_68 {
        &self.huffenc_ac1_68
    }
    ///0x774 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_69(&self) -> &HUFFENC_AC1_69 {
        &self.huffenc_ac1_69
    }
    ///0x778 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_70(&self) -> &HUFFENC_AC1_70 {
        &self.huffenc_ac1_70
    }
    ///0x77c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_71(&self) -> &HUFFENC_AC1_71 {
        &self.huffenc_ac1_71
    }
    ///0x780 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_72(&self) -> &HUFFENC_AC1_72 {
        &self.huffenc_ac1_72
    }
    ///0x784 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_73(&self) -> &HUFFENC_AC1_73 {
        &self.huffenc_ac1_73
    }
    ///0x788 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_74(&self) -> &HUFFENC_AC1_74 {
        &self.huffenc_ac1_74
    }
    ///0x78c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_75(&self) -> &HUFFENC_AC1_75 {
        &self.huffenc_ac1_75
    }
    ///0x790 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_76(&self) -> &HUFFENC_AC1_76 {
        &self.huffenc_ac1_76
    }
    ///0x794 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_77(&self) -> &HUFFENC_AC1_77 {
        &self.huffenc_ac1_77
    }
    ///0x798 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_78(&self) -> &HUFFENC_AC1_78 {
        &self.huffenc_ac1_78
    }
    ///0x79c - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_79(&self) -> &HUFFENC_AC1_79 {
        &self.huffenc_ac1_79
    }
    ///0x7a0 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_80(&self) -> &HUFFENC_AC1_80 {
        &self.huffenc_ac1_80
    }
    ///0x7a4 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_81(&self) -> &HUFFENC_AC1_81 {
        &self.huffenc_ac1_81
    }
    ///0x7a8 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_82(&self) -> &HUFFENC_AC1_82 {
        &self.huffenc_ac1_82
    }
    ///0x7ac - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_83(&self) -> &HUFFENC_AC1_83 {
        &self.huffenc_ac1_83
    }
    ///0x7b0 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_84(&self) -> &HUFFENC_AC1_84 {
        &self.huffenc_ac1_84
    }
    ///0x7b4 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_85(&self) -> &HUFFENC_AC1_85 {
        &self.huffenc_ac1_85
    }
    ///0x7b8 - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_86(&self) -> &HUFFENC_AC1_86 {
        &self.huffenc_ac1_86
    }
    ///0x7bc - JPEG Huffman encoder AC1
    #[inline(always)]
    pub const fn huffenc_ac1_87(&self) -> &HUFFENC_AC1_87 {
        &self.huffenc_ac1_87
    }
    ///0x7c0 - JPEG Huffman encoder DC0
    #[inline(always)]
    pub const fn huffenc_dc0_0(&self) -> &HUFFENC_DC0_0 {
        &self.huffenc_dc0_0
    }
    ///0x7c4 - JPEG Huffman encoder DC0
    #[inline(always)]
    pub const fn huffenc_dc0_1(&self) -> &HUFFENC_DC0_1 {
        &self.huffenc_dc0_1
    }
    ///0x7c8 - JPEG Huffman encoder DC0
    #[inline(always)]
    pub const fn huffenc_dc0_2(&self) -> &HUFFENC_DC0_2 {
        &self.huffenc_dc0_2
    }
    ///0x7cc - JPEG Huffman encoder DC0
    #[inline(always)]
    pub const fn huffenc_dc0_3(&self) -> &HUFFENC_DC0_3 {
        &self.huffenc_dc0_3
    }
    ///0x7d0 - JPEG Huffman encoder DC0
    #[inline(always)]
    pub const fn huffenc_dc0_4(&self) -> &HUFFENC_DC0_4 {
        &self.huffenc_dc0_4
    }
    ///0x7d4 - JPEG Huffman encoder DC0
    #[inline(always)]
    pub const fn huffenc_dc0_5(&self) -> &HUFFENC_DC0_5 {
        &self.huffenc_dc0_5
    }
    ///0x7d8 - JPEG Huffman encoder DC0
    #[inline(always)]
    pub const fn huffenc_dc0_6(&self) -> &HUFFENC_DC0_6 {
        &self.huffenc_dc0_6
    }
    ///0x7dc - JPEG Huffman encoder DC0
    #[inline(always)]
    pub const fn huffenc_dc0_7(&self) -> &HUFFENC_DC0_7 {
        &self.huffenc_dc0_7
    }
    ///0x7e0 - JPEG Huffman encoder DC1
    #[inline(always)]
    pub const fn huffenc_dc1_0(&self) -> &HUFFENC_DC1_0 {
        &self.huffenc_dc1_0
    }
    ///0x7e4 - JPEG Huffman encoder DC1
    #[inline(always)]
    pub const fn huffenc_dc1_1(&self) -> &HUFFENC_DC1_1 {
        &self.huffenc_dc1_1
    }
    ///0x7e8 - JPEG Huffman encoder DC1
    #[inline(always)]
    pub const fn huffenc_dc1_2(&self) -> &HUFFENC_DC1_2 {
        &self.huffenc_dc1_2
    }
    ///0x7ec - JPEG Huffman encoder DC1
    #[inline(always)]
    pub const fn huffenc_dc1_3(&self) -> &HUFFENC_DC1_3 {
        &self.huffenc_dc1_3
    }
    ///0x7f0 - JPEG Huffman encoder DC1
    #[inline(always)]
    pub const fn huffenc_dc1_4(&self) -> &HUFFENC_DC1_4 {
        &self.huffenc_dc1_4
    }
    ///0x7f4 - JPEG Huffman encoder DC1
    #[inline(always)]
    pub const fn huffenc_dc1_5(&self) -> &HUFFENC_DC1_5 {
        &self.huffenc_dc1_5
    }
    ///0x7f8 - JPEG Huffman encoder DC1
    #[inline(always)]
    pub const fn huffenc_dc1_6(&self) -> &HUFFENC_DC1_6 {
        &self.huffenc_dc1_6
    }
    ///0x7fc - JPEG Huffman encoder DC1
    #[inline(always)]
    pub const fn huffenc_dc1_7(&self) -> &HUFFENC_DC1_7 {
        &self.huffenc_dc1_7
    }
}
/**CONFR0 (w) register accessor: JPEG codec control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CONFR0)

For information about available fields see [`mod@confr0`] module*/
pub type CONFR0 = crate::Reg<confr0::CONFR0rs>;
///JPEG codec control register
pub mod confr0;
/**CONFR1 (rw) register accessor: JPEG codec configuration register 1

You can [`read`](crate::Reg::read) this register and get [`confr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CONFR1)

For information about available fields see [`mod@confr1`] module*/
pub type CONFR1 = crate::Reg<confr1::CONFR1rs>;
///JPEG codec configuration register 1
pub mod confr1;
/**CONFR2 (rw) register accessor: JPEG codec configuration register 2

You can [`read`](crate::Reg::read) this register and get [`confr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CONFR2)

For information about available fields see [`mod@confr2`] module*/
pub type CONFR2 = crate::Reg<confr2::CONFR2rs>;
///JPEG codec configuration register 2
pub mod confr2;
/**CONFR3 (rw) register accessor: JPEG codec configuration register 3

You can [`read`](crate::Reg::read) this register and get [`confr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CONFR3)

For information about available fields see [`mod@confr3`] module*/
pub type CONFR3 = crate::Reg<confr3::CONFR3rs>;
///JPEG codec configuration register 3
pub mod confr3;
/**CONFR4 (rw) register accessor: JPEG codec configuration register 4

You can [`read`](crate::Reg::read) this register and get [`confr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CONFR4)

For information about available fields see [`mod@confr4`] module*/
pub type CONFR4 = crate::Reg<confr4::CONFR4rs>;
///JPEG codec configuration register 4
pub mod confr4;
/**CONFR5 (rw) register accessor: JPEG codec configuration register 5

You can [`read`](crate::Reg::read) this register and get [`confr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CONFR5)

For information about available fields see [`mod@confr5`] module*/
pub type CONFR5 = crate::Reg<confr5::CONFR5rs>;
///JPEG codec configuration register 5
pub mod confr5;
/**CONFR6 (rw) register accessor: JPEG codec configuration register 6

You can [`read`](crate::Reg::read) this register and get [`confr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CONFR6)

For information about available fields see [`mod@confr6`] module*/
pub type CONFR6 = crate::Reg<confr6::CONFR6rs>;
///JPEG codec configuration register 6
pub mod confr6;
/**CONFR7 (rw) register accessor: JPEG codec configuration register 7

You can [`read`](crate::Reg::read) this register and get [`confr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CONFR7)

For information about available fields see [`mod@confr7`] module*/
pub type CONFR7 = crate::Reg<confr7::CONFR7rs>;
///JPEG codec configuration register 7
pub mod confr7;
/**CR (rw) register accessor: JPEG control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///JPEG control register
pub mod cr;
/**SR (r) register accessor: JPEG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///JPEG status register
pub mod sr;
/**CFR (rw) register accessor: JPEG clear flag register

You can [`read`](crate::Reg::read) this register and get [`cfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CFR)

For information about available fields see [`mod@cfr`] module*/
pub type CFR = crate::Reg<cfr::CFRrs>;
///JPEG clear flag register
pub mod cfr;
/**DIR (w) register accessor: JPEG data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DIR)

For information about available fields see [`mod@dir`] module*/
pub type DIR = crate::Reg<dir::DIRrs>;
///JPEG data input register
pub mod dir;
/**DOR (r) register accessor: JPEG data output register

You can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DOR)

For information about available fields see [`mod@dor`] module*/
pub type DOR = crate::Reg<dor::DORrs>;
///JPEG data output register
pub mod dor;
/**QMEM0_0 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_0)

For information about available fields see [`mod@qmem0_0`] module*/
pub type QMEM0_0 = crate::Reg<qmem0_0::QMEM0_0rs>;
///JPEG quantization memory 0
pub mod qmem0_0;
/**QMEM0_1 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_1)

For information about available fields see [`mod@qmem0_1`] module*/
pub type QMEM0_1 = crate::Reg<qmem0_1::QMEM0_1rs>;
///JPEG quantization memory 0
pub mod qmem0_1;
/**QMEM0_2 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_2)

For information about available fields see [`mod@qmem0_2`] module*/
pub type QMEM0_2 = crate::Reg<qmem0_2::QMEM0_2rs>;
///JPEG quantization memory 0
pub mod qmem0_2;
/**QMEM0_3 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_3)

For information about available fields see [`mod@qmem0_3`] module*/
pub type QMEM0_3 = crate::Reg<qmem0_3::QMEM0_3rs>;
///JPEG quantization memory 0
pub mod qmem0_3;
/**QMEM0_4 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_4)

For information about available fields see [`mod@qmem0_4`] module*/
pub type QMEM0_4 = crate::Reg<qmem0_4::QMEM0_4rs>;
///JPEG quantization memory 0
pub mod qmem0_4;
/**QMEM0_5 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_5)

For information about available fields see [`mod@qmem0_5`] module*/
pub type QMEM0_5 = crate::Reg<qmem0_5::QMEM0_5rs>;
///JPEG quantization memory 0
pub mod qmem0_5;
/**QMEM0_6 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_6)

For information about available fields see [`mod@qmem0_6`] module*/
pub type QMEM0_6 = crate::Reg<qmem0_6::QMEM0_6rs>;
///JPEG quantization memory 0
pub mod qmem0_6;
/**QMEM0_7 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_7)

For information about available fields see [`mod@qmem0_7`] module*/
pub type QMEM0_7 = crate::Reg<qmem0_7::QMEM0_7rs>;
///JPEG quantization memory 0
pub mod qmem0_7;
/**QMEM0_8 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_8)

For information about available fields see [`mod@qmem0_8`] module*/
pub type QMEM0_8 = crate::Reg<qmem0_8::QMEM0_8rs>;
///JPEG quantization memory 0
pub mod qmem0_8;
/**QMEM0_9 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_9)

For information about available fields see [`mod@qmem0_9`] module*/
pub type QMEM0_9 = crate::Reg<qmem0_9::QMEM0_9rs>;
///JPEG quantization memory 0
pub mod qmem0_9;
/**QMEM0_10 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_10)

For information about available fields see [`mod@qmem0_10`] module*/
pub type QMEM0_10 = crate::Reg<qmem0_10::QMEM0_10rs>;
///JPEG quantization memory 0
pub mod qmem0_10;
/**QMEM0_11 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_11)

For information about available fields see [`mod@qmem0_11`] module*/
pub type QMEM0_11 = crate::Reg<qmem0_11::QMEM0_11rs>;
///JPEG quantization memory 0
pub mod qmem0_11;
/**QMEM0_12 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_12)

For information about available fields see [`mod@qmem0_12`] module*/
pub type QMEM0_12 = crate::Reg<qmem0_12::QMEM0_12rs>;
///JPEG quantization memory 0
pub mod qmem0_12;
/**QMEM0_13 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_13)

For information about available fields see [`mod@qmem0_13`] module*/
pub type QMEM0_13 = crate::Reg<qmem0_13::QMEM0_13rs>;
///JPEG quantization memory 0
pub mod qmem0_13;
/**QMEM0_14 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_14)

For information about available fields see [`mod@qmem0_14`] module*/
pub type QMEM0_14 = crate::Reg<qmem0_14::QMEM0_14rs>;
///JPEG quantization memory 0
pub mod qmem0_14;
/**QMEM0_15 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_15)

For information about available fields see [`mod@qmem0_15`] module*/
pub type QMEM0_15 = crate::Reg<qmem0_15::QMEM0_15rs>;
///JPEG quantization memory 0
pub mod qmem0_15;
/**QMEM1_0 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_0)

For information about available fields see [`mod@qmem1_0`] module*/
pub type QMEM1_0 = crate::Reg<qmem1_0::QMEM1_0rs>;
///JPEG quantization memory 1
pub mod qmem1_0;
/**QMEM1_1 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_1)

For information about available fields see [`mod@qmem1_1`] module*/
pub type QMEM1_1 = crate::Reg<qmem1_1::QMEM1_1rs>;
///JPEG quantization memory 1
pub mod qmem1_1;
/**QMEM1_2 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_2)

For information about available fields see [`mod@qmem1_2`] module*/
pub type QMEM1_2 = crate::Reg<qmem1_2::QMEM1_2rs>;
///JPEG quantization memory 1
pub mod qmem1_2;
/**QMEM1_3 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_3)

For information about available fields see [`mod@qmem1_3`] module*/
pub type QMEM1_3 = crate::Reg<qmem1_3::QMEM1_3rs>;
///JPEG quantization memory 1
pub mod qmem1_3;
/**QMEM1_4 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_4)

For information about available fields see [`mod@qmem1_4`] module*/
pub type QMEM1_4 = crate::Reg<qmem1_4::QMEM1_4rs>;
///JPEG quantization memory 1
pub mod qmem1_4;
/**QMEM1_5 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_5)

For information about available fields see [`mod@qmem1_5`] module*/
pub type QMEM1_5 = crate::Reg<qmem1_5::QMEM1_5rs>;
///JPEG quantization memory 1
pub mod qmem1_5;
/**QMEM1_6 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_6)

For information about available fields see [`mod@qmem1_6`] module*/
pub type QMEM1_6 = crate::Reg<qmem1_6::QMEM1_6rs>;
///JPEG quantization memory 1
pub mod qmem1_6;
/**QMEM1_7 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_7)

For information about available fields see [`mod@qmem1_7`] module*/
pub type QMEM1_7 = crate::Reg<qmem1_7::QMEM1_7rs>;
///JPEG quantization memory 1
pub mod qmem1_7;
/**QMEM1_8 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_8)

For information about available fields see [`mod@qmem1_8`] module*/
pub type QMEM1_8 = crate::Reg<qmem1_8::QMEM1_8rs>;
///JPEG quantization memory 1
pub mod qmem1_8;
/**QMEM1_9 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_9)

For information about available fields see [`mod@qmem1_9`] module*/
pub type QMEM1_9 = crate::Reg<qmem1_9::QMEM1_9rs>;
///JPEG quantization memory 1
pub mod qmem1_9;
/**QMEM1_10 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_10)

For information about available fields see [`mod@qmem1_10`] module*/
pub type QMEM1_10 = crate::Reg<qmem1_10::QMEM1_10rs>;
///JPEG quantization memory 1
pub mod qmem1_10;
/**QMEM1_11 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_11)

For information about available fields see [`mod@qmem1_11`] module*/
pub type QMEM1_11 = crate::Reg<qmem1_11::QMEM1_11rs>;
///JPEG quantization memory 1
pub mod qmem1_11;
/**QMEM1_12 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_12)

For information about available fields see [`mod@qmem1_12`] module*/
pub type QMEM1_12 = crate::Reg<qmem1_12::QMEM1_12rs>;
///JPEG quantization memory 1
pub mod qmem1_12;
/**QMEM1_13 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_13)

For information about available fields see [`mod@qmem1_13`] module*/
pub type QMEM1_13 = crate::Reg<qmem1_13::QMEM1_13rs>;
///JPEG quantization memory 1
pub mod qmem1_13;
/**QMEM1_14 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_14)

For information about available fields see [`mod@qmem1_14`] module*/
pub type QMEM1_14 = crate::Reg<qmem1_14::QMEM1_14rs>;
///JPEG quantization memory 1
pub mod qmem1_14;
/**QMEM1_15 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_15)

For information about available fields see [`mod@qmem1_15`] module*/
pub type QMEM1_15 = crate::Reg<qmem1_15::QMEM1_15rs>;
///JPEG quantization memory 1
pub mod qmem1_15;
/**QMEM2_0 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_0)

For information about available fields see [`mod@qmem2_0`] module*/
pub type QMEM2_0 = crate::Reg<qmem2_0::QMEM2_0rs>;
///JPEG quantization memory 2
pub mod qmem2_0;
/**QMEM2_1 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_1)

For information about available fields see [`mod@qmem2_1`] module*/
pub type QMEM2_1 = crate::Reg<qmem2_1::QMEM2_1rs>;
///JPEG quantization memory 2
pub mod qmem2_1;
/**QMEM2_2 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_2)

For information about available fields see [`mod@qmem2_2`] module*/
pub type QMEM2_2 = crate::Reg<qmem2_2::QMEM2_2rs>;
///JPEG quantization memory 2
pub mod qmem2_2;
/**QMEM2_3 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_3)

For information about available fields see [`mod@qmem2_3`] module*/
pub type QMEM2_3 = crate::Reg<qmem2_3::QMEM2_3rs>;
///JPEG quantization memory 2
pub mod qmem2_3;
/**QMEM2_4 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_4)

For information about available fields see [`mod@qmem2_4`] module*/
pub type QMEM2_4 = crate::Reg<qmem2_4::QMEM2_4rs>;
///JPEG quantization memory 2
pub mod qmem2_4;
/**QMEM2_5 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_5)

For information about available fields see [`mod@qmem2_5`] module*/
pub type QMEM2_5 = crate::Reg<qmem2_5::QMEM2_5rs>;
///JPEG quantization memory 2
pub mod qmem2_5;
/**QMEM2_6 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_6)

For information about available fields see [`mod@qmem2_6`] module*/
pub type QMEM2_6 = crate::Reg<qmem2_6::QMEM2_6rs>;
///JPEG quantization memory 2
pub mod qmem2_6;
/**QMEM2_7 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_7)

For information about available fields see [`mod@qmem2_7`] module*/
pub type QMEM2_7 = crate::Reg<qmem2_7::QMEM2_7rs>;
///JPEG quantization memory 2
pub mod qmem2_7;
/**QMEM2_8 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_8)

For information about available fields see [`mod@qmem2_8`] module*/
pub type QMEM2_8 = crate::Reg<qmem2_8::QMEM2_8rs>;
///JPEG quantization memory 2
pub mod qmem2_8;
/**QMEM2_9 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_9)

For information about available fields see [`mod@qmem2_9`] module*/
pub type QMEM2_9 = crate::Reg<qmem2_9::QMEM2_9rs>;
///JPEG quantization memory 2
pub mod qmem2_9;
/**QMEM2_10 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_10)

For information about available fields see [`mod@qmem2_10`] module*/
pub type QMEM2_10 = crate::Reg<qmem2_10::QMEM2_10rs>;
///JPEG quantization memory 2
pub mod qmem2_10;
/**QMEM2_11 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_11)

For information about available fields see [`mod@qmem2_11`] module*/
pub type QMEM2_11 = crate::Reg<qmem2_11::QMEM2_11rs>;
///JPEG quantization memory 2
pub mod qmem2_11;
/**QMEM2_12 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_12)

For information about available fields see [`mod@qmem2_12`] module*/
pub type QMEM2_12 = crate::Reg<qmem2_12::QMEM2_12rs>;
///JPEG quantization memory 2
pub mod qmem2_12;
/**QMEM2_13 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_13)

For information about available fields see [`mod@qmem2_13`] module*/
pub type QMEM2_13 = crate::Reg<qmem2_13::QMEM2_13rs>;
///JPEG quantization memory 2
pub mod qmem2_13;
/**QMEM2_14 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_14)

For information about available fields see [`mod@qmem2_14`] module*/
pub type QMEM2_14 = crate::Reg<qmem2_14::QMEM2_14rs>;
///JPEG quantization memory 2
pub mod qmem2_14;
/**QMEM2_15 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM2_15)

For information about available fields see [`mod@qmem2_15`] module*/
pub type QMEM2_15 = crate::Reg<qmem2_15::QMEM2_15rs>;
///JPEG quantization memory 2
pub mod qmem2_15;
/**QMEM3_0 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_0)

For information about available fields see [`mod@qmem3_0`] module*/
pub type QMEM3_0 = crate::Reg<qmem3_0::QMEM3_0rs>;
///JPEG quantization memory 3
pub mod qmem3_0;
/**QMEM3_1 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_1)

For information about available fields see [`mod@qmem3_1`] module*/
pub type QMEM3_1 = crate::Reg<qmem3_1::QMEM3_1rs>;
///JPEG quantization memory 3
pub mod qmem3_1;
/**QMEM3_2 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_2)

For information about available fields see [`mod@qmem3_2`] module*/
pub type QMEM3_2 = crate::Reg<qmem3_2::QMEM3_2rs>;
///JPEG quantization memory 3
pub mod qmem3_2;
/**QMEM3_3 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_3)

For information about available fields see [`mod@qmem3_3`] module*/
pub type QMEM3_3 = crate::Reg<qmem3_3::QMEM3_3rs>;
///JPEG quantization memory 3
pub mod qmem3_3;
/**QMEM3_4 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_4)

For information about available fields see [`mod@qmem3_4`] module*/
pub type QMEM3_4 = crate::Reg<qmem3_4::QMEM3_4rs>;
///JPEG quantization memory 3
pub mod qmem3_4;
/**QMEM3_5 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_5)

For information about available fields see [`mod@qmem3_5`] module*/
pub type QMEM3_5 = crate::Reg<qmem3_5::QMEM3_5rs>;
///JPEG quantization memory 3
pub mod qmem3_5;
/**QMEM3_6 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_6)

For information about available fields see [`mod@qmem3_6`] module*/
pub type QMEM3_6 = crate::Reg<qmem3_6::QMEM3_6rs>;
///JPEG quantization memory 3
pub mod qmem3_6;
/**QMEM3_7 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_7)

For information about available fields see [`mod@qmem3_7`] module*/
pub type QMEM3_7 = crate::Reg<qmem3_7::QMEM3_7rs>;
///JPEG quantization memory 3
pub mod qmem3_7;
/**QMEM3_8 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_8)

For information about available fields see [`mod@qmem3_8`] module*/
pub type QMEM3_8 = crate::Reg<qmem3_8::QMEM3_8rs>;
///JPEG quantization memory 3
pub mod qmem3_8;
/**QMEM3_9 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_9)

For information about available fields see [`mod@qmem3_9`] module*/
pub type QMEM3_9 = crate::Reg<qmem3_9::QMEM3_9rs>;
///JPEG quantization memory 3
pub mod qmem3_9;
/**QMEM3_10 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_10)

For information about available fields see [`mod@qmem3_10`] module*/
pub type QMEM3_10 = crate::Reg<qmem3_10::QMEM3_10rs>;
///JPEG quantization memory 3
pub mod qmem3_10;
/**QMEM3_11 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_11)

For information about available fields see [`mod@qmem3_11`] module*/
pub type QMEM3_11 = crate::Reg<qmem3_11::QMEM3_11rs>;
///JPEG quantization memory 3
pub mod qmem3_11;
/**QMEM3_12 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_12)

For information about available fields see [`mod@qmem3_12`] module*/
pub type QMEM3_12 = crate::Reg<qmem3_12::QMEM3_12rs>;
///JPEG quantization memory 3
pub mod qmem3_12;
/**QMEM3_13 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_13)

For information about available fields see [`mod@qmem3_13`] module*/
pub type QMEM3_13 = crate::Reg<qmem3_13::QMEM3_13rs>;
///JPEG quantization memory 3
pub mod qmem3_13;
/**QMEM3_14 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_14)

For information about available fields see [`mod@qmem3_14`] module*/
pub type QMEM3_14 = crate::Reg<qmem3_14::QMEM3_14rs>;
///JPEG quantization memory 3
pub mod qmem3_14;
/**QMEM3_15 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_15)

For information about available fields see [`mod@qmem3_15`] module*/
pub type QMEM3_15 = crate::Reg<qmem3_15::QMEM3_15rs>;
///JPEG quantization memory 3
pub mod qmem3_15;
/**HUFFMIN0_0 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN0_0)

For information about available fields see [`mod@huffmin0_0`] module*/
pub type HUFFMIN0_0 = crate::Reg<huffmin0_0::HUFFMIN0_0rs>;
///JPEG Huffman min
pub mod huffmin0_0;
/**HUFFMIN0_1 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN0_1)

For information about available fields see [`mod@huffmin0_1`] module*/
pub type HUFFMIN0_1 = crate::Reg<huffmin0_1::HUFFMIN0_1rs>;
///JPEG Huffman min
pub mod huffmin0_1;
/**HUFFMIN0_2 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN0_2)

For information about available fields see [`mod@huffmin0_2`] module*/
pub type HUFFMIN0_2 = crate::Reg<huffmin0_2::HUFFMIN0_2rs>;
///JPEG Huffman min
pub mod huffmin0_2;
/**HUFFMIN0_3 (rw) register accessor: JPEG Huffman min 0

You can [`read`](crate::Reg::read) this register and get [`huffmin0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN0_3)

For information about available fields see [`mod@huffmin0_3`] module*/
pub type HUFFMIN0_3 = crate::Reg<huffmin0_3::HUFFMIN0_3rs>;
///JPEG Huffman min 0
pub mod huffmin0_3;
/**HUFFMIN1_0 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN1_0)

For information about available fields see [`mod@huffmin1_0`] module*/
pub type HUFFMIN1_0 = crate::Reg<huffmin1_0::HUFFMIN1_0rs>;
///JPEG Huffman min
pub mod huffmin1_0;
/**HUFFMIN1_1 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN1_1)

For information about available fields see [`mod@huffmin1_1`] module*/
pub type HUFFMIN1_1 = crate::Reg<huffmin1_1::HUFFMIN1_1rs>;
///JPEG Huffman min
pub mod huffmin1_1;
/**HUFFMIN1_2 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN1_2)

For information about available fields see [`mod@huffmin1_2`] module*/
pub type HUFFMIN1_2 = crate::Reg<huffmin1_2::HUFFMIN1_2rs>;
///JPEG Huffman min
pub mod huffmin1_2;
/**HUFFMIN1_3 (rw) register accessor: JPEG Huffman min 1

You can [`read`](crate::Reg::read) this register and get [`huffmin1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN1_3)

For information about available fields see [`mod@huffmin1_3`] module*/
pub type HUFFMIN1_3 = crate::Reg<huffmin1_3::HUFFMIN1_3rs>;
///JPEG Huffman min 1
pub mod huffmin1_3;
/**HUFFMIN2_0 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN2_0)

For information about available fields see [`mod@huffmin2_0`] module*/
pub type HUFFMIN2_0 = crate::Reg<huffmin2_0::HUFFMIN2_0rs>;
///JPEG Huffman min
pub mod huffmin2_0;
/**HUFFMIN2_1 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN2_1)

For information about available fields see [`mod@huffmin2_1`] module*/
pub type HUFFMIN2_1 = crate::Reg<huffmin2_1::HUFFMIN2_1rs>;
///JPEG Huffman min
pub mod huffmin2_1;
/**HUFFMIN2_2 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin2_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin2_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN2_2)

For information about available fields see [`mod@huffmin2_2`] module*/
pub type HUFFMIN2_2 = crate::Reg<huffmin2_2::HUFFMIN2_2rs>;
///JPEG Huffman min
pub mod huffmin2_2;
/**HUFFMIN2_3 (rw) register accessor: JPEG Huffman min 2

You can [`read`](crate::Reg::read) this register and get [`huffmin2_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin2_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN2_3)

For information about available fields see [`mod@huffmin2_3`] module*/
pub type HUFFMIN2_3 = crate::Reg<huffmin2_3::HUFFMIN2_3rs>;
///JPEG Huffman min 2
pub mod huffmin2_3;
/**HUFFMIN3_0 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN3_0)

For information about available fields see [`mod@huffmin3_0`] module*/
pub type HUFFMIN3_0 = crate::Reg<huffmin3_0::HUFFMIN3_0rs>;
///JPEG Huffman min
pub mod huffmin3_0;
/**HUFFMIN3_1 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin3_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin3_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN3_1)

For information about available fields see [`mod@huffmin3_1`] module*/
pub type HUFFMIN3_1 = crate::Reg<huffmin3_1::HUFFMIN3_1rs>;
///JPEG Huffman min
pub mod huffmin3_1;
/**HUFFMIN3_2 (rw) register accessor: JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin3_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin3_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN3_2)

For information about available fields see [`mod@huffmin3_2`] module*/
pub type HUFFMIN3_2 = crate::Reg<huffmin3_2::HUFFMIN3_2rs>;
///JPEG Huffman min
pub mod huffmin3_2;
/**HUFFMIN3_3 (rw) register accessor: JPEG Huffman min 3

You can [`read`](crate::Reg::read) this register and get [`huffmin3_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin3_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN3_3)

For information about available fields see [`mod@huffmin3_3`] module*/
pub type HUFFMIN3_3 = crate::Reg<huffmin3_3::HUFFMIN3_3rs>;
///JPEG Huffman min 3
pub mod huffmin3_3;
/**HUFFBASE0 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE0)

For information about available fields see [`mod@huffbase0`] module*/
pub type HUFFBASE0 = crate::Reg<huffbase0::HUFFBASE0rs>;
///JPEG Huffman base
pub mod huffbase0;
/**HUFFBASE1 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE1)

For information about available fields see [`mod@huffbase1`] module*/
pub type HUFFBASE1 = crate::Reg<huffbase1::HUFFBASE1rs>;
///JPEG Huffman base
pub mod huffbase1;
/**HUFFBASE2 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE2)

For information about available fields see [`mod@huffbase2`] module*/
pub type HUFFBASE2 = crate::Reg<huffbase2::HUFFBASE2rs>;
///JPEG Huffman base
pub mod huffbase2;
/**HUFFBASE3 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE3)

For information about available fields see [`mod@huffbase3`] module*/
pub type HUFFBASE3 = crate::Reg<huffbase3::HUFFBASE3rs>;
///JPEG Huffman base
pub mod huffbase3;
/**HUFFBASE4 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE4)

For information about available fields see [`mod@huffbase4`] module*/
pub type HUFFBASE4 = crate::Reg<huffbase4::HUFFBASE4rs>;
///JPEG Huffman base
pub mod huffbase4;
/**HUFFBASE5 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE5)

For information about available fields see [`mod@huffbase5`] module*/
pub type HUFFBASE5 = crate::Reg<huffbase5::HUFFBASE5rs>;
///JPEG Huffman base
pub mod huffbase5;
/**HUFFBASE6 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE6)

For information about available fields see [`mod@huffbase6`] module*/
pub type HUFFBASE6 = crate::Reg<huffbase6::HUFFBASE6rs>;
///JPEG Huffman base
pub mod huffbase6;
/**HUFFBASE7 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE7)

For information about available fields see [`mod@huffbase7`] module*/
pub type HUFFBASE7 = crate::Reg<huffbase7::HUFFBASE7rs>;
///JPEG Huffman base
pub mod huffbase7;
/**HUFFBASE8 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE8)

For information about available fields see [`mod@huffbase8`] module*/
pub type HUFFBASE8 = crate::Reg<huffbase8::HUFFBASE8rs>;
///JPEG Huffman base
pub mod huffbase8;
/**HUFFBASE9 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE9)

For information about available fields see [`mod@huffbase9`] module*/
pub type HUFFBASE9 = crate::Reg<huffbase9::HUFFBASE9rs>;
///JPEG Huffman base
pub mod huffbase9;
/**HUFFBASE10 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE10)

For information about available fields see [`mod@huffbase10`] module*/
pub type HUFFBASE10 = crate::Reg<huffbase10::HUFFBASE10rs>;
///JPEG Huffman base
pub mod huffbase10;
/**HUFFBASE11 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE11)

For information about available fields see [`mod@huffbase11`] module*/
pub type HUFFBASE11 = crate::Reg<huffbase11::HUFFBASE11rs>;
///JPEG Huffman base
pub mod huffbase11;
/**HUFFBASE12 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE12)

For information about available fields see [`mod@huffbase12`] module*/
pub type HUFFBASE12 = crate::Reg<huffbase12::HUFFBASE12rs>;
///JPEG Huffman base
pub mod huffbase12;
/**HUFFBASE13 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE13)

For information about available fields see [`mod@huffbase13`] module*/
pub type HUFFBASE13 = crate::Reg<huffbase13::HUFFBASE13rs>;
///JPEG Huffman base
pub mod huffbase13;
/**HUFFBASE14 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE14)

For information about available fields see [`mod@huffbase14`] module*/
pub type HUFFBASE14 = crate::Reg<huffbase14::HUFFBASE14rs>;
///JPEG Huffman base
pub mod huffbase14;
/**HUFFBASE15 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE15)

For information about available fields see [`mod@huffbase15`] module*/
pub type HUFFBASE15 = crate::Reg<huffbase15::HUFFBASE15rs>;
///JPEG Huffman base
pub mod huffbase15;
/**HUFFBASE16 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE16)

For information about available fields see [`mod@huffbase16`] module*/
pub type HUFFBASE16 = crate::Reg<huffbase16::HUFFBASE16rs>;
///JPEG Huffman base
pub mod huffbase16;
/**HUFFBASE17 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE17)

For information about available fields see [`mod@huffbase17`] module*/
pub type HUFFBASE17 = crate::Reg<huffbase17::HUFFBASE17rs>;
///JPEG Huffman base
pub mod huffbase17;
/**HUFFBASE18 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE18)

For information about available fields see [`mod@huffbase18`] module*/
pub type HUFFBASE18 = crate::Reg<huffbase18::HUFFBASE18rs>;
///JPEG Huffman base
pub mod huffbase18;
/**HUFFBASE19 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE19)

For information about available fields see [`mod@huffbase19`] module*/
pub type HUFFBASE19 = crate::Reg<huffbase19::HUFFBASE19rs>;
///JPEG Huffman base
pub mod huffbase19;
/**HUFFBASE20 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE20)

For information about available fields see [`mod@huffbase20`] module*/
pub type HUFFBASE20 = crate::Reg<huffbase20::HUFFBASE20rs>;
///JPEG Huffman base
pub mod huffbase20;
/**HUFFBASE21 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE21)

For information about available fields see [`mod@huffbase21`] module*/
pub type HUFFBASE21 = crate::Reg<huffbase21::HUFFBASE21rs>;
///JPEG Huffman base
pub mod huffbase21;
/**HUFFBASE22 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE22)

For information about available fields see [`mod@huffbase22`] module*/
pub type HUFFBASE22 = crate::Reg<huffbase22::HUFFBASE22rs>;
///JPEG Huffman base
pub mod huffbase22;
/**HUFFBASE23 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE23)

For information about available fields see [`mod@huffbase23`] module*/
pub type HUFFBASE23 = crate::Reg<huffbase23::HUFFBASE23rs>;
///JPEG Huffman base
pub mod huffbase23;
/**HUFFBASE24 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE24)

For information about available fields see [`mod@huffbase24`] module*/
pub type HUFFBASE24 = crate::Reg<huffbase24::HUFFBASE24rs>;
///JPEG Huffman base
pub mod huffbase24;
/**HUFFBASE25 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE25)

For information about available fields see [`mod@huffbase25`] module*/
pub type HUFFBASE25 = crate::Reg<huffbase25::HUFFBASE25rs>;
///JPEG Huffman base
pub mod huffbase25;
/**HUFFBASE26 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE26)

For information about available fields see [`mod@huffbase26`] module*/
pub type HUFFBASE26 = crate::Reg<huffbase26::HUFFBASE26rs>;
///JPEG Huffman base
pub mod huffbase26;
/**HUFFBASE27 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE27)

For information about available fields see [`mod@huffbase27`] module*/
pub type HUFFBASE27 = crate::Reg<huffbase27::HUFFBASE27rs>;
///JPEG Huffman base
pub mod huffbase27;
/**HUFFBASE28 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE28)

For information about available fields see [`mod@huffbase28`] module*/
pub type HUFFBASE28 = crate::Reg<huffbase28::HUFFBASE28rs>;
///JPEG Huffman base
pub mod huffbase28;
/**HUFFBASE29 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE29)

For information about available fields see [`mod@huffbase29`] module*/
pub type HUFFBASE29 = crate::Reg<huffbase29::HUFFBASE29rs>;
///JPEG Huffman base
pub mod huffbase29;
/**HUFFBASE30 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE30)

For information about available fields see [`mod@huffbase30`] module*/
pub type HUFFBASE30 = crate::Reg<huffbase30::HUFFBASE30rs>;
///JPEG Huffman base
pub mod huffbase30;
/**HUFFBASE31 (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE31)

For information about available fields see [`mod@huffbase31`] module*/
pub type HUFFBASE31 = crate::Reg<huffbase31::HUFFBASE31rs>;
///JPEG Huffman base
pub mod huffbase31;
/**HUFFSYMB0 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB0)

For information about available fields see [`mod@huffsymb0`] module*/
pub type HUFFSYMB0 = crate::Reg<huffsymb0::HUFFSYMB0rs>;
///JPEG Huffman symbol
pub mod huffsymb0;
/**HUFFSYMB1 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB1)

For information about available fields see [`mod@huffsymb1`] module*/
pub type HUFFSYMB1 = crate::Reg<huffsymb1::HUFFSYMB1rs>;
///JPEG Huffman symbol
pub mod huffsymb1;
/**HUFFSYMB2 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB2)

For information about available fields see [`mod@huffsymb2`] module*/
pub type HUFFSYMB2 = crate::Reg<huffsymb2::HUFFSYMB2rs>;
///JPEG Huffman symbol
pub mod huffsymb2;
/**HUFFSYMB3 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB3)

For information about available fields see [`mod@huffsymb3`] module*/
pub type HUFFSYMB3 = crate::Reg<huffsymb3::HUFFSYMB3rs>;
///JPEG Huffman symbol
pub mod huffsymb3;
/**HUFFSYMB4 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB4)

For information about available fields see [`mod@huffsymb4`] module*/
pub type HUFFSYMB4 = crate::Reg<huffsymb4::HUFFSYMB4rs>;
///JPEG Huffman symbol
pub mod huffsymb4;
/**HUFFSYMB5 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB5)

For information about available fields see [`mod@huffsymb5`] module*/
pub type HUFFSYMB5 = crate::Reg<huffsymb5::HUFFSYMB5rs>;
///JPEG Huffman symbol
pub mod huffsymb5;
/**HUFFSYMB6 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB6)

For information about available fields see [`mod@huffsymb6`] module*/
pub type HUFFSYMB6 = crate::Reg<huffsymb6::HUFFSYMB6rs>;
///JPEG Huffman symbol
pub mod huffsymb6;
/**HUFFSYMB7 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB7)

For information about available fields see [`mod@huffsymb7`] module*/
pub type HUFFSYMB7 = crate::Reg<huffsymb7::HUFFSYMB7rs>;
///JPEG Huffman symbol
pub mod huffsymb7;
/**HUFFSYMB8 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB8)

For information about available fields see [`mod@huffsymb8`] module*/
pub type HUFFSYMB8 = crate::Reg<huffsymb8::HUFFSYMB8rs>;
///JPEG Huffman symbol
pub mod huffsymb8;
/**HUFFSYMB9 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB9)

For information about available fields see [`mod@huffsymb9`] module*/
pub type HUFFSYMB9 = crate::Reg<huffsymb9::HUFFSYMB9rs>;
///JPEG Huffman symbol
pub mod huffsymb9;
/**HUFFSYMB10 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB10)

For information about available fields see [`mod@huffsymb10`] module*/
pub type HUFFSYMB10 = crate::Reg<huffsymb10::HUFFSYMB10rs>;
///JPEG Huffman symbol
pub mod huffsymb10;
/**HUFFSYMB11 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB11)

For information about available fields see [`mod@huffsymb11`] module*/
pub type HUFFSYMB11 = crate::Reg<huffsymb11::HUFFSYMB11rs>;
///JPEG Huffman symbol
pub mod huffsymb11;
/**HUFFSYMB12 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB12)

For information about available fields see [`mod@huffsymb12`] module*/
pub type HUFFSYMB12 = crate::Reg<huffsymb12::HUFFSYMB12rs>;
///JPEG Huffman symbol
pub mod huffsymb12;
/**HUFFSYMB13 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB13)

For information about available fields see [`mod@huffsymb13`] module*/
pub type HUFFSYMB13 = crate::Reg<huffsymb13::HUFFSYMB13rs>;
///JPEG Huffman symbol
pub mod huffsymb13;
/**HUFFSYMB14 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB14)

For information about available fields see [`mod@huffsymb14`] module*/
pub type HUFFSYMB14 = crate::Reg<huffsymb14::HUFFSYMB14rs>;
///JPEG Huffman symbol
pub mod huffsymb14;
/**HUFFSYMB15 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB15)

For information about available fields see [`mod@huffsymb15`] module*/
pub type HUFFSYMB15 = crate::Reg<huffsymb15::HUFFSYMB15rs>;
///JPEG Huffman symbol
pub mod huffsymb15;
/**HUFFSYMB16 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB16)

For information about available fields see [`mod@huffsymb16`] module*/
pub type HUFFSYMB16 = crate::Reg<huffsymb16::HUFFSYMB16rs>;
///JPEG Huffman symbol
pub mod huffsymb16;
/**HUFFSYMB17 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB17)

For information about available fields see [`mod@huffsymb17`] module*/
pub type HUFFSYMB17 = crate::Reg<huffsymb17::HUFFSYMB17rs>;
///JPEG Huffman symbol
pub mod huffsymb17;
/**HUFFSYMB18 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB18)

For information about available fields see [`mod@huffsymb18`] module*/
pub type HUFFSYMB18 = crate::Reg<huffsymb18::HUFFSYMB18rs>;
///JPEG Huffman symbol
pub mod huffsymb18;
/**HUFFSYMB19 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB19)

For information about available fields see [`mod@huffsymb19`] module*/
pub type HUFFSYMB19 = crate::Reg<huffsymb19::HUFFSYMB19rs>;
///JPEG Huffman symbol
pub mod huffsymb19;
/**HUFFSYMB20 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB20)

For information about available fields see [`mod@huffsymb20`] module*/
pub type HUFFSYMB20 = crate::Reg<huffsymb20::HUFFSYMB20rs>;
///JPEG Huffman symbol
pub mod huffsymb20;
/**HUFFSYMB21 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB21)

For information about available fields see [`mod@huffsymb21`] module*/
pub type HUFFSYMB21 = crate::Reg<huffsymb21::HUFFSYMB21rs>;
///JPEG Huffman symbol
pub mod huffsymb21;
/**HUFFSYMB22 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB22)

For information about available fields see [`mod@huffsymb22`] module*/
pub type HUFFSYMB22 = crate::Reg<huffsymb22::HUFFSYMB22rs>;
///JPEG Huffman symbol
pub mod huffsymb22;
/**HUFFSYMB23 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB23)

For information about available fields see [`mod@huffsymb23`] module*/
pub type HUFFSYMB23 = crate::Reg<huffsymb23::HUFFSYMB23rs>;
///JPEG Huffman symbol
pub mod huffsymb23;
/**HUFFSYMB24 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB24)

For information about available fields see [`mod@huffsymb24`] module*/
pub type HUFFSYMB24 = crate::Reg<huffsymb24::HUFFSYMB24rs>;
///JPEG Huffman symbol
pub mod huffsymb24;
/**HUFFSYMB25 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB25)

For information about available fields see [`mod@huffsymb25`] module*/
pub type HUFFSYMB25 = crate::Reg<huffsymb25::HUFFSYMB25rs>;
///JPEG Huffman symbol
pub mod huffsymb25;
/**HUFFSYMB26 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB26)

For information about available fields see [`mod@huffsymb26`] module*/
pub type HUFFSYMB26 = crate::Reg<huffsymb26::HUFFSYMB26rs>;
///JPEG Huffman symbol
pub mod huffsymb26;
/**HUFFSYMB27 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB27)

For information about available fields see [`mod@huffsymb27`] module*/
pub type HUFFSYMB27 = crate::Reg<huffsymb27::HUFFSYMB27rs>;
///JPEG Huffman symbol
pub mod huffsymb27;
/**HUFFSYMB28 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB28)

For information about available fields see [`mod@huffsymb28`] module*/
pub type HUFFSYMB28 = crate::Reg<huffsymb28::HUFFSYMB28rs>;
///JPEG Huffman symbol
pub mod huffsymb28;
/**HUFFSYMB29 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB29)

For information about available fields see [`mod@huffsymb29`] module*/
pub type HUFFSYMB29 = crate::Reg<huffsymb29::HUFFSYMB29rs>;
///JPEG Huffman symbol
pub mod huffsymb29;
/**HUFFSYMB30 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB30)

For information about available fields see [`mod@huffsymb30`] module*/
pub type HUFFSYMB30 = crate::Reg<huffsymb30::HUFFSYMB30rs>;
///JPEG Huffman symbol
pub mod huffsymb30;
/**HUFFSYMB31 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB31)

For information about available fields see [`mod@huffsymb31`] module*/
pub type HUFFSYMB31 = crate::Reg<huffsymb31::HUFFSYMB31rs>;
///JPEG Huffman symbol
pub mod huffsymb31;
/**HUFFSYMB32 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB32)

For information about available fields see [`mod@huffsymb32`] module*/
pub type HUFFSYMB32 = crate::Reg<huffsymb32::HUFFSYMB32rs>;
///JPEG Huffman symbol
pub mod huffsymb32;
/**HUFFSYMB33 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB33)

For information about available fields see [`mod@huffsymb33`] module*/
pub type HUFFSYMB33 = crate::Reg<huffsymb33::HUFFSYMB33rs>;
///JPEG Huffman symbol
pub mod huffsymb33;
/**HUFFSYMB34 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB34)

For information about available fields see [`mod@huffsymb34`] module*/
pub type HUFFSYMB34 = crate::Reg<huffsymb34::HUFFSYMB34rs>;
///JPEG Huffman symbol
pub mod huffsymb34;
/**HUFFSYMB35 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB35)

For information about available fields see [`mod@huffsymb35`] module*/
pub type HUFFSYMB35 = crate::Reg<huffsymb35::HUFFSYMB35rs>;
///JPEG Huffman symbol
pub mod huffsymb35;
/**HUFFSYMB36 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB36)

For information about available fields see [`mod@huffsymb36`] module*/
pub type HUFFSYMB36 = crate::Reg<huffsymb36::HUFFSYMB36rs>;
///JPEG Huffman symbol
pub mod huffsymb36;
/**HUFFSYMB37 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB37)

For information about available fields see [`mod@huffsymb37`] module*/
pub type HUFFSYMB37 = crate::Reg<huffsymb37::HUFFSYMB37rs>;
///JPEG Huffman symbol
pub mod huffsymb37;
/**HUFFSYMB38 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB38)

For information about available fields see [`mod@huffsymb38`] module*/
pub type HUFFSYMB38 = crate::Reg<huffsymb38::HUFFSYMB38rs>;
///JPEG Huffman symbol
pub mod huffsymb38;
/**HUFFSYMB39 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB39)

For information about available fields see [`mod@huffsymb39`] module*/
pub type HUFFSYMB39 = crate::Reg<huffsymb39::HUFFSYMB39rs>;
///JPEG Huffman symbol
pub mod huffsymb39;
/**HUFFSYMB40 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB40)

For information about available fields see [`mod@huffsymb40`] module*/
pub type HUFFSYMB40 = crate::Reg<huffsymb40::HUFFSYMB40rs>;
///JPEG Huffman symbol
pub mod huffsymb40;
/**HUFFSYMB41 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB41)

For information about available fields see [`mod@huffsymb41`] module*/
pub type HUFFSYMB41 = crate::Reg<huffsymb41::HUFFSYMB41rs>;
///JPEG Huffman symbol
pub mod huffsymb41;
/**HUFFSYMB42 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB42)

For information about available fields see [`mod@huffsymb42`] module*/
pub type HUFFSYMB42 = crate::Reg<huffsymb42::HUFFSYMB42rs>;
///JPEG Huffman symbol
pub mod huffsymb42;
/**HUFFSYMB43 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB43)

For information about available fields see [`mod@huffsymb43`] module*/
pub type HUFFSYMB43 = crate::Reg<huffsymb43::HUFFSYMB43rs>;
///JPEG Huffman symbol
pub mod huffsymb43;
/**HUFFSYMB44 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB44)

For information about available fields see [`mod@huffsymb44`] module*/
pub type HUFFSYMB44 = crate::Reg<huffsymb44::HUFFSYMB44rs>;
///JPEG Huffman symbol
pub mod huffsymb44;
/**HUFFSYMB45 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB45)

For information about available fields see [`mod@huffsymb45`] module*/
pub type HUFFSYMB45 = crate::Reg<huffsymb45::HUFFSYMB45rs>;
///JPEG Huffman symbol
pub mod huffsymb45;
/**HUFFSYMB46 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB46)

For information about available fields see [`mod@huffsymb46`] module*/
pub type HUFFSYMB46 = crate::Reg<huffsymb46::HUFFSYMB46rs>;
///JPEG Huffman symbol
pub mod huffsymb46;
/**HUFFSYMB47 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB47)

For information about available fields see [`mod@huffsymb47`] module*/
pub type HUFFSYMB47 = crate::Reg<huffsymb47::HUFFSYMB47rs>;
///JPEG Huffman symbol
pub mod huffsymb47;
/**HUFFSYMB48 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB48)

For information about available fields see [`mod@huffsymb48`] module*/
pub type HUFFSYMB48 = crate::Reg<huffsymb48::HUFFSYMB48rs>;
///JPEG Huffman symbol
pub mod huffsymb48;
/**HUFFSYMB49 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB49)

For information about available fields see [`mod@huffsymb49`] module*/
pub type HUFFSYMB49 = crate::Reg<huffsymb49::HUFFSYMB49rs>;
///JPEG Huffman symbol
pub mod huffsymb49;
/**HUFFSYMB50 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB50)

For information about available fields see [`mod@huffsymb50`] module*/
pub type HUFFSYMB50 = crate::Reg<huffsymb50::HUFFSYMB50rs>;
///JPEG Huffman symbol
pub mod huffsymb50;
/**HUFFSYMB51 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB51)

For information about available fields see [`mod@huffsymb51`] module*/
pub type HUFFSYMB51 = crate::Reg<huffsymb51::HUFFSYMB51rs>;
///JPEG Huffman symbol
pub mod huffsymb51;
/**HUFFSYMB52 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB52)

For information about available fields see [`mod@huffsymb52`] module*/
pub type HUFFSYMB52 = crate::Reg<huffsymb52::HUFFSYMB52rs>;
///JPEG Huffman symbol
pub mod huffsymb52;
/**HUFFSYMB53 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB53)

For information about available fields see [`mod@huffsymb53`] module*/
pub type HUFFSYMB53 = crate::Reg<huffsymb53::HUFFSYMB53rs>;
///JPEG Huffman symbol
pub mod huffsymb53;
/**HUFFSYMB54 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB54)

For information about available fields see [`mod@huffsymb54`] module*/
pub type HUFFSYMB54 = crate::Reg<huffsymb54::HUFFSYMB54rs>;
///JPEG Huffman symbol
pub mod huffsymb54;
/**HUFFSYMB55 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB55)

For information about available fields see [`mod@huffsymb55`] module*/
pub type HUFFSYMB55 = crate::Reg<huffsymb55::HUFFSYMB55rs>;
///JPEG Huffman symbol
pub mod huffsymb55;
/**HUFFSYMB56 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB56)

For information about available fields see [`mod@huffsymb56`] module*/
pub type HUFFSYMB56 = crate::Reg<huffsymb56::HUFFSYMB56rs>;
///JPEG Huffman symbol
pub mod huffsymb56;
/**HUFFSYMB57 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB57)

For information about available fields see [`mod@huffsymb57`] module*/
pub type HUFFSYMB57 = crate::Reg<huffsymb57::HUFFSYMB57rs>;
///JPEG Huffman symbol
pub mod huffsymb57;
/**HUFFSYMB58 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB58)

For information about available fields see [`mod@huffsymb58`] module*/
pub type HUFFSYMB58 = crate::Reg<huffsymb58::HUFFSYMB58rs>;
///JPEG Huffman symbol
pub mod huffsymb58;
/**HUFFSYMB59 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB59)

For information about available fields see [`mod@huffsymb59`] module*/
pub type HUFFSYMB59 = crate::Reg<huffsymb59::HUFFSYMB59rs>;
///JPEG Huffman symbol
pub mod huffsymb59;
/**HUFFSYMB60 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB60)

For information about available fields see [`mod@huffsymb60`] module*/
pub type HUFFSYMB60 = crate::Reg<huffsymb60::HUFFSYMB60rs>;
///JPEG Huffman symbol
pub mod huffsymb60;
/**HUFFSYMB61 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB61)

For information about available fields see [`mod@huffsymb61`] module*/
pub type HUFFSYMB61 = crate::Reg<huffsymb61::HUFFSYMB61rs>;
///JPEG Huffman symbol
pub mod huffsymb61;
/**HUFFSYMB62 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB62)

For information about available fields see [`mod@huffsymb62`] module*/
pub type HUFFSYMB62 = crate::Reg<huffsymb62::HUFFSYMB62rs>;
///JPEG Huffman symbol
pub mod huffsymb62;
/**HUFFSYMB63 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB63)

For information about available fields see [`mod@huffsymb63`] module*/
pub type HUFFSYMB63 = crate::Reg<huffsymb63::HUFFSYMB63rs>;
///JPEG Huffman symbol
pub mod huffsymb63;
/**HUFFSYMB64 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB64)

For information about available fields see [`mod@huffsymb64`] module*/
pub type HUFFSYMB64 = crate::Reg<huffsymb64::HUFFSYMB64rs>;
///JPEG Huffman symbol
pub mod huffsymb64;
/**HUFFSYMB65 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB65)

For information about available fields see [`mod@huffsymb65`] module*/
pub type HUFFSYMB65 = crate::Reg<huffsymb65::HUFFSYMB65rs>;
///JPEG Huffman symbol
pub mod huffsymb65;
/**HUFFSYMB66 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB66)

For information about available fields see [`mod@huffsymb66`] module*/
pub type HUFFSYMB66 = crate::Reg<huffsymb66::HUFFSYMB66rs>;
///JPEG Huffman symbol
pub mod huffsymb66;
/**HUFFSYMB67 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB67)

For information about available fields see [`mod@huffsymb67`] module*/
pub type HUFFSYMB67 = crate::Reg<huffsymb67::HUFFSYMB67rs>;
///JPEG Huffman symbol
pub mod huffsymb67;
/**HUFFSYMB68 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB68)

For information about available fields see [`mod@huffsymb68`] module*/
pub type HUFFSYMB68 = crate::Reg<huffsymb68::HUFFSYMB68rs>;
///JPEG Huffman symbol
pub mod huffsymb68;
/**HUFFSYMB69 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB69)

For information about available fields see [`mod@huffsymb69`] module*/
pub type HUFFSYMB69 = crate::Reg<huffsymb69::HUFFSYMB69rs>;
///JPEG Huffman symbol
pub mod huffsymb69;
/**HUFFSYMB70 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB70)

For information about available fields see [`mod@huffsymb70`] module*/
pub type HUFFSYMB70 = crate::Reg<huffsymb70::HUFFSYMB70rs>;
///JPEG Huffman symbol
pub mod huffsymb70;
/**HUFFSYMB71 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB71)

For information about available fields see [`mod@huffsymb71`] module*/
pub type HUFFSYMB71 = crate::Reg<huffsymb71::HUFFSYMB71rs>;
///JPEG Huffman symbol
pub mod huffsymb71;
/**HUFFSYMB72 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB72)

For information about available fields see [`mod@huffsymb72`] module*/
pub type HUFFSYMB72 = crate::Reg<huffsymb72::HUFFSYMB72rs>;
///JPEG Huffman symbol
pub mod huffsymb72;
/**HUFFSYMB73 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB73)

For information about available fields see [`mod@huffsymb73`] module*/
pub type HUFFSYMB73 = crate::Reg<huffsymb73::HUFFSYMB73rs>;
///JPEG Huffman symbol
pub mod huffsymb73;
/**HUFFSYMB74 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB74)

For information about available fields see [`mod@huffsymb74`] module*/
pub type HUFFSYMB74 = crate::Reg<huffsymb74::HUFFSYMB74rs>;
///JPEG Huffman symbol
pub mod huffsymb74;
/**HUFFSYMB75 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB75)

For information about available fields see [`mod@huffsymb75`] module*/
pub type HUFFSYMB75 = crate::Reg<huffsymb75::HUFFSYMB75rs>;
///JPEG Huffman symbol
pub mod huffsymb75;
/**HUFFSYMB76 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB76)

For information about available fields see [`mod@huffsymb76`] module*/
pub type HUFFSYMB76 = crate::Reg<huffsymb76::HUFFSYMB76rs>;
///JPEG Huffman symbol
pub mod huffsymb76;
/**HUFFSYMB77 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB77)

For information about available fields see [`mod@huffsymb77`] module*/
pub type HUFFSYMB77 = crate::Reg<huffsymb77::HUFFSYMB77rs>;
///JPEG Huffman symbol
pub mod huffsymb77;
/**HUFFSYMB78 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB78)

For information about available fields see [`mod@huffsymb78`] module*/
pub type HUFFSYMB78 = crate::Reg<huffsymb78::HUFFSYMB78rs>;
///JPEG Huffman symbol
pub mod huffsymb78;
/**HUFFSYMB79 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB79)

For information about available fields see [`mod@huffsymb79`] module*/
pub type HUFFSYMB79 = crate::Reg<huffsymb79::HUFFSYMB79rs>;
///JPEG Huffman symbol
pub mod huffsymb79;
/**HUFFSYMB80 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB80)

For information about available fields see [`mod@huffsymb80`] module*/
pub type HUFFSYMB80 = crate::Reg<huffsymb80::HUFFSYMB80rs>;
///JPEG Huffman symbol
pub mod huffsymb80;
/**HUFFSYMB81 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB81)

For information about available fields see [`mod@huffsymb81`] module*/
pub type HUFFSYMB81 = crate::Reg<huffsymb81::HUFFSYMB81rs>;
///JPEG Huffman symbol
pub mod huffsymb81;
/**HUFFSYMB82 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB82)

For information about available fields see [`mod@huffsymb82`] module*/
pub type HUFFSYMB82 = crate::Reg<huffsymb82::HUFFSYMB82rs>;
///JPEG Huffman symbol
pub mod huffsymb82;
/**HUFFSYMB83 (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB83)

For information about available fields see [`mod@huffsymb83`] module*/
pub type HUFFSYMB83 = crate::Reg<huffsymb83::HUFFSYMB83rs>;
///JPEG Huffman symbol
pub mod huffsymb83;
/**DHTMEM0 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM0)

For information about available fields see [`mod@dhtmem0`] module*/
pub type DHTMEM0 = crate::Reg<dhtmem0::DHTMEM0rs>;
///JPEG DHT memory
pub mod dhtmem0;
/**DHTMEM1 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM1)

For information about available fields see [`mod@dhtmem1`] module*/
pub type DHTMEM1 = crate::Reg<dhtmem1::DHTMEM1rs>;
///JPEG DHT memory
pub mod dhtmem1;
/**DHTMEM2 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM2)

For information about available fields see [`mod@dhtmem2`] module*/
pub type DHTMEM2 = crate::Reg<dhtmem2::DHTMEM2rs>;
///JPEG DHT memory
pub mod dhtmem2;
/**DHTMEM3 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM3)

For information about available fields see [`mod@dhtmem3`] module*/
pub type DHTMEM3 = crate::Reg<dhtmem3::DHTMEM3rs>;
///JPEG DHT memory
pub mod dhtmem3;
/**DHTMEM4 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM4)

For information about available fields see [`mod@dhtmem4`] module*/
pub type DHTMEM4 = crate::Reg<dhtmem4::DHTMEM4rs>;
///JPEG DHT memory
pub mod dhtmem4;
/**DHTMEM5 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM5)

For information about available fields see [`mod@dhtmem5`] module*/
pub type DHTMEM5 = crate::Reg<dhtmem5::DHTMEM5rs>;
///JPEG DHT memory
pub mod dhtmem5;
/**DHTMEM6 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM6)

For information about available fields see [`mod@dhtmem6`] module*/
pub type DHTMEM6 = crate::Reg<dhtmem6::DHTMEM6rs>;
///JPEG DHT memory
pub mod dhtmem6;
/**DHTMEM7 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM7)

For information about available fields see [`mod@dhtmem7`] module*/
pub type DHTMEM7 = crate::Reg<dhtmem7::DHTMEM7rs>;
///JPEG DHT memory
pub mod dhtmem7;
/**DHTMEM8 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM8)

For information about available fields see [`mod@dhtmem8`] module*/
pub type DHTMEM8 = crate::Reg<dhtmem8::DHTMEM8rs>;
///JPEG DHT memory
pub mod dhtmem8;
/**DHTMEM9 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM9)

For information about available fields see [`mod@dhtmem9`] module*/
pub type DHTMEM9 = crate::Reg<dhtmem9::DHTMEM9rs>;
///JPEG DHT memory
pub mod dhtmem9;
/**DHTMEM10 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM10)

For information about available fields see [`mod@dhtmem10`] module*/
pub type DHTMEM10 = crate::Reg<dhtmem10::DHTMEM10rs>;
///JPEG DHT memory
pub mod dhtmem10;
/**DHTMEM11 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM11)

For information about available fields see [`mod@dhtmem11`] module*/
pub type DHTMEM11 = crate::Reg<dhtmem11::DHTMEM11rs>;
///JPEG DHT memory
pub mod dhtmem11;
/**DHTMEM12 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM12)

For information about available fields see [`mod@dhtmem12`] module*/
pub type DHTMEM12 = crate::Reg<dhtmem12::DHTMEM12rs>;
///JPEG DHT memory
pub mod dhtmem12;
/**DHTMEM13 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM13)

For information about available fields see [`mod@dhtmem13`] module*/
pub type DHTMEM13 = crate::Reg<dhtmem13::DHTMEM13rs>;
///JPEG DHT memory
pub mod dhtmem13;
/**DHTMEM14 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM14)

For information about available fields see [`mod@dhtmem14`] module*/
pub type DHTMEM14 = crate::Reg<dhtmem14::DHTMEM14rs>;
///JPEG DHT memory
pub mod dhtmem14;
/**DHTMEM15 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM15)

For information about available fields see [`mod@dhtmem15`] module*/
pub type DHTMEM15 = crate::Reg<dhtmem15::DHTMEM15rs>;
///JPEG DHT memory
pub mod dhtmem15;
/**DHTMEM16 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM16)

For information about available fields see [`mod@dhtmem16`] module*/
pub type DHTMEM16 = crate::Reg<dhtmem16::DHTMEM16rs>;
///JPEG DHT memory
pub mod dhtmem16;
/**DHTMEM17 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM17)

For information about available fields see [`mod@dhtmem17`] module*/
pub type DHTMEM17 = crate::Reg<dhtmem17::DHTMEM17rs>;
///JPEG DHT memory
pub mod dhtmem17;
/**DHTMEM18 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM18)

For information about available fields see [`mod@dhtmem18`] module*/
pub type DHTMEM18 = crate::Reg<dhtmem18::DHTMEM18rs>;
///JPEG DHT memory
pub mod dhtmem18;
/**DHTMEM19 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM19)

For information about available fields see [`mod@dhtmem19`] module*/
pub type DHTMEM19 = crate::Reg<dhtmem19::DHTMEM19rs>;
///JPEG DHT memory
pub mod dhtmem19;
/**DHTMEM20 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM20)

For information about available fields see [`mod@dhtmem20`] module*/
pub type DHTMEM20 = crate::Reg<dhtmem20::DHTMEM20rs>;
///JPEG DHT memory
pub mod dhtmem20;
/**DHTMEM21 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM21)

For information about available fields see [`mod@dhtmem21`] module*/
pub type DHTMEM21 = crate::Reg<dhtmem21::DHTMEM21rs>;
///JPEG DHT memory
pub mod dhtmem21;
/**DHTMEM22 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM22)

For information about available fields see [`mod@dhtmem22`] module*/
pub type DHTMEM22 = crate::Reg<dhtmem22::DHTMEM22rs>;
///JPEG DHT memory
pub mod dhtmem22;
/**DHTMEM23 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM23)

For information about available fields see [`mod@dhtmem23`] module*/
pub type DHTMEM23 = crate::Reg<dhtmem23::DHTMEM23rs>;
///JPEG DHT memory
pub mod dhtmem23;
/**DHTMEM24 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM24)

For information about available fields see [`mod@dhtmem24`] module*/
pub type DHTMEM24 = crate::Reg<dhtmem24::DHTMEM24rs>;
///JPEG DHT memory
pub mod dhtmem24;
/**DHTMEM25 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM25)

For information about available fields see [`mod@dhtmem25`] module*/
pub type DHTMEM25 = crate::Reg<dhtmem25::DHTMEM25rs>;
///JPEG DHT memory
pub mod dhtmem25;
/**DHTMEM26 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM26)

For information about available fields see [`mod@dhtmem26`] module*/
pub type DHTMEM26 = crate::Reg<dhtmem26::DHTMEM26rs>;
///JPEG DHT memory
pub mod dhtmem26;
/**DHTMEM27 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM27)

For information about available fields see [`mod@dhtmem27`] module*/
pub type DHTMEM27 = crate::Reg<dhtmem27::DHTMEM27rs>;
///JPEG DHT memory
pub mod dhtmem27;
/**DHTMEM28 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM28)

For information about available fields see [`mod@dhtmem28`] module*/
pub type DHTMEM28 = crate::Reg<dhtmem28::DHTMEM28rs>;
///JPEG DHT memory
pub mod dhtmem28;
/**DHTMEM29 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM29)

For information about available fields see [`mod@dhtmem29`] module*/
pub type DHTMEM29 = crate::Reg<dhtmem29::DHTMEM29rs>;
///JPEG DHT memory
pub mod dhtmem29;
/**DHTMEM30 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM30)

For information about available fields see [`mod@dhtmem30`] module*/
pub type DHTMEM30 = crate::Reg<dhtmem30::DHTMEM30rs>;
///JPEG DHT memory
pub mod dhtmem30;
/**DHTMEM31 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM31)

For information about available fields see [`mod@dhtmem31`] module*/
pub type DHTMEM31 = crate::Reg<dhtmem31::DHTMEM31rs>;
///JPEG DHT memory
pub mod dhtmem31;
/**DHTMEM32 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM32)

For information about available fields see [`mod@dhtmem32`] module*/
pub type DHTMEM32 = crate::Reg<dhtmem32::DHTMEM32rs>;
///JPEG DHT memory
pub mod dhtmem32;
/**DHTMEM33 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM33)

For information about available fields see [`mod@dhtmem33`] module*/
pub type DHTMEM33 = crate::Reg<dhtmem33::DHTMEM33rs>;
///JPEG DHT memory
pub mod dhtmem33;
/**DHTMEM34 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM34)

For information about available fields see [`mod@dhtmem34`] module*/
pub type DHTMEM34 = crate::Reg<dhtmem34::DHTMEM34rs>;
///JPEG DHT memory
pub mod dhtmem34;
/**DHTMEM35 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM35)

For information about available fields see [`mod@dhtmem35`] module*/
pub type DHTMEM35 = crate::Reg<dhtmem35::DHTMEM35rs>;
///JPEG DHT memory
pub mod dhtmem35;
/**DHTMEM36 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM36)

For information about available fields see [`mod@dhtmem36`] module*/
pub type DHTMEM36 = crate::Reg<dhtmem36::DHTMEM36rs>;
///JPEG DHT memory
pub mod dhtmem36;
/**DHTMEM37 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM37)

For information about available fields see [`mod@dhtmem37`] module*/
pub type DHTMEM37 = crate::Reg<dhtmem37::DHTMEM37rs>;
///JPEG DHT memory
pub mod dhtmem37;
/**DHTMEM38 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM38)

For information about available fields see [`mod@dhtmem38`] module*/
pub type DHTMEM38 = crate::Reg<dhtmem38::DHTMEM38rs>;
///JPEG DHT memory
pub mod dhtmem38;
/**DHTMEM39 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM39)

For information about available fields see [`mod@dhtmem39`] module*/
pub type DHTMEM39 = crate::Reg<dhtmem39::DHTMEM39rs>;
///JPEG DHT memory
pub mod dhtmem39;
/**DHTMEM40 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM40)

For information about available fields see [`mod@dhtmem40`] module*/
pub type DHTMEM40 = crate::Reg<dhtmem40::DHTMEM40rs>;
///JPEG DHT memory
pub mod dhtmem40;
/**DHTMEM41 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM41)

For information about available fields see [`mod@dhtmem41`] module*/
pub type DHTMEM41 = crate::Reg<dhtmem41::DHTMEM41rs>;
///JPEG DHT memory
pub mod dhtmem41;
/**DHTMEM42 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM42)

For information about available fields see [`mod@dhtmem42`] module*/
pub type DHTMEM42 = crate::Reg<dhtmem42::DHTMEM42rs>;
///JPEG DHT memory
pub mod dhtmem42;
/**DHTMEM43 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM43)

For information about available fields see [`mod@dhtmem43`] module*/
pub type DHTMEM43 = crate::Reg<dhtmem43::DHTMEM43rs>;
///JPEG DHT memory
pub mod dhtmem43;
/**DHTMEM44 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM44)

For information about available fields see [`mod@dhtmem44`] module*/
pub type DHTMEM44 = crate::Reg<dhtmem44::DHTMEM44rs>;
///JPEG DHT memory
pub mod dhtmem44;
/**DHTMEM45 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM45)

For information about available fields see [`mod@dhtmem45`] module*/
pub type DHTMEM45 = crate::Reg<dhtmem45::DHTMEM45rs>;
///JPEG DHT memory
pub mod dhtmem45;
/**DHTMEM46 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM46)

For information about available fields see [`mod@dhtmem46`] module*/
pub type DHTMEM46 = crate::Reg<dhtmem46::DHTMEM46rs>;
///JPEG DHT memory
pub mod dhtmem46;
/**DHTMEM47 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM47)

For information about available fields see [`mod@dhtmem47`] module*/
pub type DHTMEM47 = crate::Reg<dhtmem47::DHTMEM47rs>;
///JPEG DHT memory
pub mod dhtmem47;
/**DHTMEM48 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM48)

For information about available fields see [`mod@dhtmem48`] module*/
pub type DHTMEM48 = crate::Reg<dhtmem48::DHTMEM48rs>;
///JPEG DHT memory
pub mod dhtmem48;
/**DHTMEM49 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM49)

For information about available fields see [`mod@dhtmem49`] module*/
pub type DHTMEM49 = crate::Reg<dhtmem49::DHTMEM49rs>;
///JPEG DHT memory
pub mod dhtmem49;
/**DHTMEM50 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM50)

For information about available fields see [`mod@dhtmem50`] module*/
pub type DHTMEM50 = crate::Reg<dhtmem50::DHTMEM50rs>;
///JPEG DHT memory
pub mod dhtmem50;
/**DHTMEM51 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM51)

For information about available fields see [`mod@dhtmem51`] module*/
pub type DHTMEM51 = crate::Reg<dhtmem51::DHTMEM51rs>;
///JPEG DHT memory
pub mod dhtmem51;
/**DHTMEM52 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM52)

For information about available fields see [`mod@dhtmem52`] module*/
pub type DHTMEM52 = crate::Reg<dhtmem52::DHTMEM52rs>;
///JPEG DHT memory
pub mod dhtmem52;
/**DHTMEM53 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM53)

For information about available fields see [`mod@dhtmem53`] module*/
pub type DHTMEM53 = crate::Reg<dhtmem53::DHTMEM53rs>;
///JPEG DHT memory
pub mod dhtmem53;
/**DHTMEM54 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM54)

For information about available fields see [`mod@dhtmem54`] module*/
pub type DHTMEM54 = crate::Reg<dhtmem54::DHTMEM54rs>;
///JPEG DHT memory
pub mod dhtmem54;
/**DHTMEM55 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM55)

For information about available fields see [`mod@dhtmem55`] module*/
pub type DHTMEM55 = crate::Reg<dhtmem55::DHTMEM55rs>;
///JPEG DHT memory
pub mod dhtmem55;
/**DHTMEM56 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM56)

For information about available fields see [`mod@dhtmem56`] module*/
pub type DHTMEM56 = crate::Reg<dhtmem56::DHTMEM56rs>;
///JPEG DHT memory
pub mod dhtmem56;
/**DHTMEM57 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM57)

For information about available fields see [`mod@dhtmem57`] module*/
pub type DHTMEM57 = crate::Reg<dhtmem57::DHTMEM57rs>;
///JPEG DHT memory
pub mod dhtmem57;
/**DHTMEM58 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM58)

For information about available fields see [`mod@dhtmem58`] module*/
pub type DHTMEM58 = crate::Reg<dhtmem58::DHTMEM58rs>;
///JPEG DHT memory
pub mod dhtmem58;
/**DHTMEM59 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM59)

For information about available fields see [`mod@dhtmem59`] module*/
pub type DHTMEM59 = crate::Reg<dhtmem59::DHTMEM59rs>;
///JPEG DHT memory
pub mod dhtmem59;
/**DHTMEM60 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM60)

For information about available fields see [`mod@dhtmem60`] module*/
pub type DHTMEM60 = crate::Reg<dhtmem60::DHTMEM60rs>;
///JPEG DHT memory
pub mod dhtmem60;
/**DHTMEM61 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM61)

For information about available fields see [`mod@dhtmem61`] module*/
pub type DHTMEM61 = crate::Reg<dhtmem61::DHTMEM61rs>;
///JPEG DHT memory
pub mod dhtmem61;
/**DHTMEM62 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM62)

For information about available fields see [`mod@dhtmem62`] module*/
pub type DHTMEM62 = crate::Reg<dhtmem62::DHTMEM62rs>;
///JPEG DHT memory
pub mod dhtmem62;
/**DHTMEM63 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM63)

For information about available fields see [`mod@dhtmem63`] module*/
pub type DHTMEM63 = crate::Reg<dhtmem63::DHTMEM63rs>;
///JPEG DHT memory
pub mod dhtmem63;
/**DHTMEM64 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM64)

For information about available fields see [`mod@dhtmem64`] module*/
pub type DHTMEM64 = crate::Reg<dhtmem64::DHTMEM64rs>;
///JPEG DHT memory
pub mod dhtmem64;
/**DHTMEM65 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM65)

For information about available fields see [`mod@dhtmem65`] module*/
pub type DHTMEM65 = crate::Reg<dhtmem65::DHTMEM65rs>;
///JPEG DHT memory
pub mod dhtmem65;
/**DHTMEM66 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM66)

For information about available fields see [`mod@dhtmem66`] module*/
pub type DHTMEM66 = crate::Reg<dhtmem66::DHTMEM66rs>;
///JPEG DHT memory
pub mod dhtmem66;
/**DHTMEM67 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM67)

For information about available fields see [`mod@dhtmem67`] module*/
pub type DHTMEM67 = crate::Reg<dhtmem67::DHTMEM67rs>;
///JPEG DHT memory
pub mod dhtmem67;
/**DHTMEM68 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM68)

For information about available fields see [`mod@dhtmem68`] module*/
pub type DHTMEM68 = crate::Reg<dhtmem68::DHTMEM68rs>;
///JPEG DHT memory
pub mod dhtmem68;
/**DHTMEM69 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM69)

For information about available fields see [`mod@dhtmem69`] module*/
pub type DHTMEM69 = crate::Reg<dhtmem69::DHTMEM69rs>;
///JPEG DHT memory
pub mod dhtmem69;
/**DHTMEM70 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM70)

For information about available fields see [`mod@dhtmem70`] module*/
pub type DHTMEM70 = crate::Reg<dhtmem70::DHTMEM70rs>;
///JPEG DHT memory
pub mod dhtmem70;
/**DHTMEM71 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM71)

For information about available fields see [`mod@dhtmem71`] module*/
pub type DHTMEM71 = crate::Reg<dhtmem71::DHTMEM71rs>;
///JPEG DHT memory
pub mod dhtmem71;
/**DHTMEM72 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM72)

For information about available fields see [`mod@dhtmem72`] module*/
pub type DHTMEM72 = crate::Reg<dhtmem72::DHTMEM72rs>;
///JPEG DHT memory
pub mod dhtmem72;
/**DHTMEM73 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM73)

For information about available fields see [`mod@dhtmem73`] module*/
pub type DHTMEM73 = crate::Reg<dhtmem73::DHTMEM73rs>;
///JPEG DHT memory
pub mod dhtmem73;
/**DHTMEM74 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM74)

For information about available fields see [`mod@dhtmem74`] module*/
pub type DHTMEM74 = crate::Reg<dhtmem74::DHTMEM74rs>;
///JPEG DHT memory
pub mod dhtmem74;
/**DHTMEM75 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM75)

For information about available fields see [`mod@dhtmem75`] module*/
pub type DHTMEM75 = crate::Reg<dhtmem75::DHTMEM75rs>;
///JPEG DHT memory
pub mod dhtmem75;
/**DHTMEM76 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM76)

For information about available fields see [`mod@dhtmem76`] module*/
pub type DHTMEM76 = crate::Reg<dhtmem76::DHTMEM76rs>;
///JPEG DHT memory
pub mod dhtmem76;
/**DHTMEM77 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM77)

For information about available fields see [`mod@dhtmem77`] module*/
pub type DHTMEM77 = crate::Reg<dhtmem77::DHTMEM77rs>;
///JPEG DHT memory
pub mod dhtmem77;
/**DHTMEM78 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM78)

For information about available fields see [`mod@dhtmem78`] module*/
pub type DHTMEM78 = crate::Reg<dhtmem78::DHTMEM78rs>;
///JPEG DHT memory
pub mod dhtmem78;
/**DHTMEM79 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM79)

For information about available fields see [`mod@dhtmem79`] module*/
pub type DHTMEM79 = crate::Reg<dhtmem79::DHTMEM79rs>;
///JPEG DHT memory
pub mod dhtmem79;
/**DHTMEM80 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM80)

For information about available fields see [`mod@dhtmem80`] module*/
pub type DHTMEM80 = crate::Reg<dhtmem80::DHTMEM80rs>;
///JPEG DHT memory
pub mod dhtmem80;
/**DHTMEM81 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM81)

For information about available fields see [`mod@dhtmem81`] module*/
pub type DHTMEM81 = crate::Reg<dhtmem81::DHTMEM81rs>;
///JPEG DHT memory
pub mod dhtmem81;
/**DHTMEM82 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM82)

For information about available fields see [`mod@dhtmem82`] module*/
pub type DHTMEM82 = crate::Reg<dhtmem82::DHTMEM82rs>;
///JPEG DHT memory
pub mod dhtmem82;
/**DHTMEM83 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM83)

For information about available fields see [`mod@dhtmem83`] module*/
pub type DHTMEM83 = crate::Reg<dhtmem83::DHTMEM83rs>;
///JPEG DHT memory
pub mod dhtmem83;
/**DHTMEM84 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM84)

For information about available fields see [`mod@dhtmem84`] module*/
pub type DHTMEM84 = crate::Reg<dhtmem84::DHTMEM84rs>;
///JPEG DHT memory
pub mod dhtmem84;
/**DHTMEM85 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM85)

For information about available fields see [`mod@dhtmem85`] module*/
pub type DHTMEM85 = crate::Reg<dhtmem85::DHTMEM85rs>;
///JPEG DHT memory
pub mod dhtmem85;
/**DHTMEM86 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM86)

For information about available fields see [`mod@dhtmem86`] module*/
pub type DHTMEM86 = crate::Reg<dhtmem86::DHTMEM86rs>;
///JPEG DHT memory
pub mod dhtmem86;
/**DHTMEM87 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM87)

For information about available fields see [`mod@dhtmem87`] module*/
pub type DHTMEM87 = crate::Reg<dhtmem87::DHTMEM87rs>;
///JPEG DHT memory
pub mod dhtmem87;
/**DHTMEM88 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM88)

For information about available fields see [`mod@dhtmem88`] module*/
pub type DHTMEM88 = crate::Reg<dhtmem88::DHTMEM88rs>;
///JPEG DHT memory
pub mod dhtmem88;
/**DHTMEM89 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM89)

For information about available fields see [`mod@dhtmem89`] module*/
pub type DHTMEM89 = crate::Reg<dhtmem89::DHTMEM89rs>;
///JPEG DHT memory
pub mod dhtmem89;
/**DHTMEM90 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM90)

For information about available fields see [`mod@dhtmem90`] module*/
pub type DHTMEM90 = crate::Reg<dhtmem90::DHTMEM90rs>;
///JPEG DHT memory
pub mod dhtmem90;
/**DHTMEM91 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM91)

For information about available fields see [`mod@dhtmem91`] module*/
pub type DHTMEM91 = crate::Reg<dhtmem91::DHTMEM91rs>;
///JPEG DHT memory
pub mod dhtmem91;
/**DHTMEM92 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM92)

For information about available fields see [`mod@dhtmem92`] module*/
pub type DHTMEM92 = crate::Reg<dhtmem92::DHTMEM92rs>;
///JPEG DHT memory
pub mod dhtmem92;
/**DHTMEM93 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM93)

For information about available fields see [`mod@dhtmem93`] module*/
pub type DHTMEM93 = crate::Reg<dhtmem93::DHTMEM93rs>;
///JPEG DHT memory
pub mod dhtmem93;
/**DHTMEM94 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM94)

For information about available fields see [`mod@dhtmem94`] module*/
pub type DHTMEM94 = crate::Reg<dhtmem94::DHTMEM94rs>;
///JPEG DHT memory
pub mod dhtmem94;
/**DHTMEM95 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM95)

For information about available fields see [`mod@dhtmem95`] module*/
pub type DHTMEM95 = crate::Reg<dhtmem95::DHTMEM95rs>;
///JPEG DHT memory
pub mod dhtmem95;
/**DHTMEM96 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM96)

For information about available fields see [`mod@dhtmem96`] module*/
pub type DHTMEM96 = crate::Reg<dhtmem96::DHTMEM96rs>;
///JPEG DHT memory
pub mod dhtmem96;
/**DHTMEM97 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM97)

For information about available fields see [`mod@dhtmem97`] module*/
pub type DHTMEM97 = crate::Reg<dhtmem97::DHTMEM97rs>;
///JPEG DHT memory
pub mod dhtmem97;
/**DHTMEM98 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM98)

For information about available fields see [`mod@dhtmem98`] module*/
pub type DHTMEM98 = crate::Reg<dhtmem98::DHTMEM98rs>;
///JPEG DHT memory
pub mod dhtmem98;
/**DHTMEM99 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem99::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM99)

For information about available fields see [`mod@dhtmem99`] module*/
pub type DHTMEM99 = crate::Reg<dhtmem99::DHTMEM99rs>;
///JPEG DHT memory
pub mod dhtmem99;
/**DHTMEM100 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM100)

For information about available fields see [`mod@dhtmem100`] module*/
pub type DHTMEM100 = crate::Reg<dhtmem100::DHTMEM100rs>;
///JPEG DHT memory
pub mod dhtmem100;
/**DHTMEM101 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem101::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem101::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM101)

For information about available fields see [`mod@dhtmem101`] module*/
pub type DHTMEM101 = crate::Reg<dhtmem101::DHTMEM101rs>;
///JPEG DHT memory
pub mod dhtmem101;
/**DHTMEM102 (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem102::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem102::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM102)

For information about available fields see [`mod@dhtmem102`] module*/
pub type DHTMEM102 = crate::Reg<dhtmem102::DHTMEM102rs>;
///JPEG DHT memory
pub mod dhtmem102;
/**HUFFENC_AC0_0 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_0)

For information about available fields see [`mod@huffenc_ac0_0`] module*/
pub type HUFFENC_AC0_0 = crate::Reg<huffenc_ac0_0::HUFFENC_AC0_0rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_0;
/**HUFFENC_AC0_1 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_1)

For information about available fields see [`mod@huffenc_ac0_1`] module*/
pub type HUFFENC_AC0_1 = crate::Reg<huffenc_ac0_1::HUFFENC_AC0_1rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_1;
/**HUFFENC_AC0_2 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_2)

For information about available fields see [`mod@huffenc_ac0_2`] module*/
pub type HUFFENC_AC0_2 = crate::Reg<huffenc_ac0_2::HUFFENC_AC0_2rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_2;
/**HUFFENC_AC0_3 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_3)

For information about available fields see [`mod@huffenc_ac0_3`] module*/
pub type HUFFENC_AC0_3 = crate::Reg<huffenc_ac0_3::HUFFENC_AC0_3rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_3;
/**HUFFENC_AC0_4 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_4)

For information about available fields see [`mod@huffenc_ac0_4`] module*/
pub type HUFFENC_AC0_4 = crate::Reg<huffenc_ac0_4::HUFFENC_AC0_4rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_4;
/**HUFFENC_AC0_5 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_5)

For information about available fields see [`mod@huffenc_ac0_5`] module*/
pub type HUFFENC_AC0_5 = crate::Reg<huffenc_ac0_5::HUFFENC_AC0_5rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_5;
/**HUFFENC_AC0_6 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_6)

For information about available fields see [`mod@huffenc_ac0_6`] module*/
pub type HUFFENC_AC0_6 = crate::Reg<huffenc_ac0_6::HUFFENC_AC0_6rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_6;
/**HUFFENC_AC0_7 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_7)

For information about available fields see [`mod@huffenc_ac0_7`] module*/
pub type HUFFENC_AC0_7 = crate::Reg<huffenc_ac0_7::HUFFENC_AC0_7rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_7;
/**HUFFENC_AC0_8 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_8)

For information about available fields see [`mod@huffenc_ac0_8`] module*/
pub type HUFFENC_AC0_8 = crate::Reg<huffenc_ac0_8::HUFFENC_AC0_8rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_8;
/**HUFFENC_AC0_9 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_9)

For information about available fields see [`mod@huffenc_ac0_9`] module*/
pub type HUFFENC_AC0_9 = crate::Reg<huffenc_ac0_9::HUFFENC_AC0_9rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_9;
/**HUFFENC_AC0_10 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_10)

For information about available fields see [`mod@huffenc_ac0_10`] module*/
pub type HUFFENC_AC0_10 = crate::Reg<huffenc_ac0_10::HUFFENC_AC0_10rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_10;
/**HUFFENC_AC0_11 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_11)

For information about available fields see [`mod@huffenc_ac0_11`] module*/
pub type HUFFENC_AC0_11 = crate::Reg<huffenc_ac0_11::HUFFENC_AC0_11rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_11;
/**HUFFENC_AC0_12 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_12)

For information about available fields see [`mod@huffenc_ac0_12`] module*/
pub type HUFFENC_AC0_12 = crate::Reg<huffenc_ac0_12::HUFFENC_AC0_12rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_12;
/**HUFFENC_AC0_13 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_13)

For information about available fields see [`mod@huffenc_ac0_13`] module*/
pub type HUFFENC_AC0_13 = crate::Reg<huffenc_ac0_13::HUFFENC_AC0_13rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_13;
/**HUFFENC_AC0_14 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_14)

For information about available fields see [`mod@huffenc_ac0_14`] module*/
pub type HUFFENC_AC0_14 = crate::Reg<huffenc_ac0_14::HUFFENC_AC0_14rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_14;
/**HUFFENC_AC0_15 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_15)

For information about available fields see [`mod@huffenc_ac0_15`] module*/
pub type HUFFENC_AC0_15 = crate::Reg<huffenc_ac0_15::HUFFENC_AC0_15rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_15;
/**HUFFENC_AC0_16 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_16)

For information about available fields see [`mod@huffenc_ac0_16`] module*/
pub type HUFFENC_AC0_16 = crate::Reg<huffenc_ac0_16::HUFFENC_AC0_16rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_16;
/**HUFFENC_AC0_17 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_17)

For information about available fields see [`mod@huffenc_ac0_17`] module*/
pub type HUFFENC_AC0_17 = crate::Reg<huffenc_ac0_17::HUFFENC_AC0_17rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_17;
/**HUFFENC_AC0_18 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_18)

For information about available fields see [`mod@huffenc_ac0_18`] module*/
pub type HUFFENC_AC0_18 = crate::Reg<huffenc_ac0_18::HUFFENC_AC0_18rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_18;
/**HUFFENC_AC0_19 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_19)

For information about available fields see [`mod@huffenc_ac0_19`] module*/
pub type HUFFENC_AC0_19 = crate::Reg<huffenc_ac0_19::HUFFENC_AC0_19rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_19;
/**HUFFENC_AC0_20 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_20)

For information about available fields see [`mod@huffenc_ac0_20`] module*/
pub type HUFFENC_AC0_20 = crate::Reg<huffenc_ac0_20::HUFFENC_AC0_20rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_20;
/**HUFFENC_AC0_21 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_21)

For information about available fields see [`mod@huffenc_ac0_21`] module*/
pub type HUFFENC_AC0_21 = crate::Reg<huffenc_ac0_21::HUFFENC_AC0_21rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_21;
/**HUFFENC_AC0_22 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_22)

For information about available fields see [`mod@huffenc_ac0_22`] module*/
pub type HUFFENC_AC0_22 = crate::Reg<huffenc_ac0_22::HUFFENC_AC0_22rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_22;
/**HUFFENC_AC0_23 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_23)

For information about available fields see [`mod@huffenc_ac0_23`] module*/
pub type HUFFENC_AC0_23 = crate::Reg<huffenc_ac0_23::HUFFENC_AC0_23rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_23;
/**HUFFENC_AC0_24 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_24)

For information about available fields see [`mod@huffenc_ac0_24`] module*/
pub type HUFFENC_AC0_24 = crate::Reg<huffenc_ac0_24::HUFFENC_AC0_24rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_24;
/**HUFFENC_AC0_25 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_25)

For information about available fields see [`mod@huffenc_ac0_25`] module*/
pub type HUFFENC_AC0_25 = crate::Reg<huffenc_ac0_25::HUFFENC_AC0_25rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_25;
/**HUFFENC_AC0_26 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_26)

For information about available fields see [`mod@huffenc_ac0_26`] module*/
pub type HUFFENC_AC0_26 = crate::Reg<huffenc_ac0_26::HUFFENC_AC0_26rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_26;
/**HUFFENC_AC0_27 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_27)

For information about available fields see [`mod@huffenc_ac0_27`] module*/
pub type HUFFENC_AC0_27 = crate::Reg<huffenc_ac0_27::HUFFENC_AC0_27rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_27;
/**HUFFENC_AC0_28 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_28)

For information about available fields see [`mod@huffenc_ac0_28`] module*/
pub type HUFFENC_AC0_28 = crate::Reg<huffenc_ac0_28::HUFFENC_AC0_28rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_28;
/**HUFFENC_AC0_29 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_29)

For information about available fields see [`mod@huffenc_ac0_29`] module*/
pub type HUFFENC_AC0_29 = crate::Reg<huffenc_ac0_29::HUFFENC_AC0_29rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_29;
/**HUFFENC_AC0_30 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_30)

For information about available fields see [`mod@huffenc_ac0_30`] module*/
pub type HUFFENC_AC0_30 = crate::Reg<huffenc_ac0_30::HUFFENC_AC0_30rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_30;
/**HUFFENC_AC0_31 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_31)

For information about available fields see [`mod@huffenc_ac0_31`] module*/
pub type HUFFENC_AC0_31 = crate::Reg<huffenc_ac0_31::HUFFENC_AC0_31rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_31;
/**HUFFENC_AC0_32 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_32)

For information about available fields see [`mod@huffenc_ac0_32`] module*/
pub type HUFFENC_AC0_32 = crate::Reg<huffenc_ac0_32::HUFFENC_AC0_32rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_32;
/**HUFFENC_AC0_33 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_33)

For information about available fields see [`mod@huffenc_ac0_33`] module*/
pub type HUFFENC_AC0_33 = crate::Reg<huffenc_ac0_33::HUFFENC_AC0_33rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_33;
/**HUFFENC_AC0_34 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_34)

For information about available fields see [`mod@huffenc_ac0_34`] module*/
pub type HUFFENC_AC0_34 = crate::Reg<huffenc_ac0_34::HUFFENC_AC0_34rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_34;
/**HUFFENC_AC0_35 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_35)

For information about available fields see [`mod@huffenc_ac0_35`] module*/
pub type HUFFENC_AC0_35 = crate::Reg<huffenc_ac0_35::HUFFENC_AC0_35rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_35;
/**HUFFENC_AC0_36 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_36)

For information about available fields see [`mod@huffenc_ac0_36`] module*/
pub type HUFFENC_AC0_36 = crate::Reg<huffenc_ac0_36::HUFFENC_AC0_36rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_36;
/**HUFFENC_AC0_37 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_37)

For information about available fields see [`mod@huffenc_ac0_37`] module*/
pub type HUFFENC_AC0_37 = crate::Reg<huffenc_ac0_37::HUFFENC_AC0_37rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_37;
/**HUFFENC_AC0_38 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_38)

For information about available fields see [`mod@huffenc_ac0_38`] module*/
pub type HUFFENC_AC0_38 = crate::Reg<huffenc_ac0_38::HUFFENC_AC0_38rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_38;
/**HUFFENC_AC0_39 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_39)

For information about available fields see [`mod@huffenc_ac0_39`] module*/
pub type HUFFENC_AC0_39 = crate::Reg<huffenc_ac0_39::HUFFENC_AC0_39rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_39;
/**HUFFENC_AC0_40 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_40)

For information about available fields see [`mod@huffenc_ac0_40`] module*/
pub type HUFFENC_AC0_40 = crate::Reg<huffenc_ac0_40::HUFFENC_AC0_40rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_40;
/**HUFFENC_AC0_41 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_41)

For information about available fields see [`mod@huffenc_ac0_41`] module*/
pub type HUFFENC_AC0_41 = crate::Reg<huffenc_ac0_41::HUFFENC_AC0_41rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_41;
/**HUFFENC_AC0_42 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_42)

For information about available fields see [`mod@huffenc_ac0_42`] module*/
pub type HUFFENC_AC0_42 = crate::Reg<huffenc_ac0_42::HUFFENC_AC0_42rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_42;
/**HUFFENC_AC0_43 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_43)

For information about available fields see [`mod@huffenc_ac0_43`] module*/
pub type HUFFENC_AC0_43 = crate::Reg<huffenc_ac0_43::HUFFENC_AC0_43rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_43;
/**HUFFENC_AC0_44 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_44)

For information about available fields see [`mod@huffenc_ac0_44`] module*/
pub type HUFFENC_AC0_44 = crate::Reg<huffenc_ac0_44::HUFFENC_AC0_44rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_44;
/**HUFFENC_AC0_45 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_45)

For information about available fields see [`mod@huffenc_ac0_45`] module*/
pub type HUFFENC_AC0_45 = crate::Reg<huffenc_ac0_45::HUFFENC_AC0_45rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_45;
/**HUFFENC_AC0_46 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_46)

For information about available fields see [`mod@huffenc_ac0_46`] module*/
pub type HUFFENC_AC0_46 = crate::Reg<huffenc_ac0_46::HUFFENC_AC0_46rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_46;
/**HUFFENC_AC0_47 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_47)

For information about available fields see [`mod@huffenc_ac0_47`] module*/
pub type HUFFENC_AC0_47 = crate::Reg<huffenc_ac0_47::HUFFENC_AC0_47rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_47;
/**HUFFENC_AC0_48 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_48)

For information about available fields see [`mod@huffenc_ac0_48`] module*/
pub type HUFFENC_AC0_48 = crate::Reg<huffenc_ac0_48::HUFFENC_AC0_48rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_48;
/**HUFFENC_AC0_49 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_49)

For information about available fields see [`mod@huffenc_ac0_49`] module*/
pub type HUFFENC_AC0_49 = crate::Reg<huffenc_ac0_49::HUFFENC_AC0_49rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_49;
/**HUFFENC_AC0_50 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_50)

For information about available fields see [`mod@huffenc_ac0_50`] module*/
pub type HUFFENC_AC0_50 = crate::Reg<huffenc_ac0_50::HUFFENC_AC0_50rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_50;
/**HUFFENC_AC0_51 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_51)

For information about available fields see [`mod@huffenc_ac0_51`] module*/
pub type HUFFENC_AC0_51 = crate::Reg<huffenc_ac0_51::HUFFENC_AC0_51rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_51;
/**HUFFENC_AC0_52 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_52)

For information about available fields see [`mod@huffenc_ac0_52`] module*/
pub type HUFFENC_AC0_52 = crate::Reg<huffenc_ac0_52::HUFFENC_AC0_52rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_52;
/**HUFFENC_AC0_53 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_53)

For information about available fields see [`mod@huffenc_ac0_53`] module*/
pub type HUFFENC_AC0_53 = crate::Reg<huffenc_ac0_53::HUFFENC_AC0_53rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_53;
/**HUFFENC_AC0_54 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_54)

For information about available fields see [`mod@huffenc_ac0_54`] module*/
pub type HUFFENC_AC0_54 = crate::Reg<huffenc_ac0_54::HUFFENC_AC0_54rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_54;
/**HUFFENC_AC0_55 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_55)

For information about available fields see [`mod@huffenc_ac0_55`] module*/
pub type HUFFENC_AC0_55 = crate::Reg<huffenc_ac0_55::HUFFENC_AC0_55rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_55;
/**HUFFENC_AC0_56 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_56)

For information about available fields see [`mod@huffenc_ac0_56`] module*/
pub type HUFFENC_AC0_56 = crate::Reg<huffenc_ac0_56::HUFFENC_AC0_56rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_56;
/**HUFFENC_AC0_57 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_57)

For information about available fields see [`mod@huffenc_ac0_57`] module*/
pub type HUFFENC_AC0_57 = crate::Reg<huffenc_ac0_57::HUFFENC_AC0_57rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_57;
/**HUFFENC_AC0_58 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_58)

For information about available fields see [`mod@huffenc_ac0_58`] module*/
pub type HUFFENC_AC0_58 = crate::Reg<huffenc_ac0_58::HUFFENC_AC0_58rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_58;
/**HUFFENC_AC0_59 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_59)

For information about available fields see [`mod@huffenc_ac0_59`] module*/
pub type HUFFENC_AC0_59 = crate::Reg<huffenc_ac0_59::HUFFENC_AC0_59rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_59;
/**HUFFENC_AC0_60 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_60)

For information about available fields see [`mod@huffenc_ac0_60`] module*/
pub type HUFFENC_AC0_60 = crate::Reg<huffenc_ac0_60::HUFFENC_AC0_60rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_60;
/**HUFFENC_AC0_61 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_61)

For information about available fields see [`mod@huffenc_ac0_61`] module*/
pub type HUFFENC_AC0_61 = crate::Reg<huffenc_ac0_61::HUFFENC_AC0_61rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_61;
/**HUFFENC_AC0_62 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_62)

For information about available fields see [`mod@huffenc_ac0_62`] module*/
pub type HUFFENC_AC0_62 = crate::Reg<huffenc_ac0_62::HUFFENC_AC0_62rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_62;
/**HUFFENC_AC0_63 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_63)

For information about available fields see [`mod@huffenc_ac0_63`] module*/
pub type HUFFENC_AC0_63 = crate::Reg<huffenc_ac0_63::HUFFENC_AC0_63rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_63;
/**HUFFENC_AC0_64 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_64)

For information about available fields see [`mod@huffenc_ac0_64`] module*/
pub type HUFFENC_AC0_64 = crate::Reg<huffenc_ac0_64::HUFFENC_AC0_64rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_64;
/**HUFFENC_AC0_65 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_65)

For information about available fields see [`mod@huffenc_ac0_65`] module*/
pub type HUFFENC_AC0_65 = crate::Reg<huffenc_ac0_65::HUFFENC_AC0_65rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_65;
/**HUFFENC_AC0_66 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_66)

For information about available fields see [`mod@huffenc_ac0_66`] module*/
pub type HUFFENC_AC0_66 = crate::Reg<huffenc_ac0_66::HUFFENC_AC0_66rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_66;
/**HUFFENC_AC0_67 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_67)

For information about available fields see [`mod@huffenc_ac0_67`] module*/
pub type HUFFENC_AC0_67 = crate::Reg<huffenc_ac0_67::HUFFENC_AC0_67rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_67;
/**HUFFENC_AC0_68 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_68)

For information about available fields see [`mod@huffenc_ac0_68`] module*/
pub type HUFFENC_AC0_68 = crate::Reg<huffenc_ac0_68::HUFFENC_AC0_68rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_68;
/**HUFFENC_AC0_69 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_69)

For information about available fields see [`mod@huffenc_ac0_69`] module*/
pub type HUFFENC_AC0_69 = crate::Reg<huffenc_ac0_69::HUFFENC_AC0_69rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_69;
/**HUFFENC_AC0_70 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_70)

For information about available fields see [`mod@huffenc_ac0_70`] module*/
pub type HUFFENC_AC0_70 = crate::Reg<huffenc_ac0_70::HUFFENC_AC0_70rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_70;
/**HUFFENC_AC0_71 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_71)

For information about available fields see [`mod@huffenc_ac0_71`] module*/
pub type HUFFENC_AC0_71 = crate::Reg<huffenc_ac0_71::HUFFENC_AC0_71rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_71;
/**HUFFENC_AC0_72 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_72)

For information about available fields see [`mod@huffenc_ac0_72`] module*/
pub type HUFFENC_AC0_72 = crate::Reg<huffenc_ac0_72::HUFFENC_AC0_72rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_72;
/**HUFFENC_AC0_73 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_73)

For information about available fields see [`mod@huffenc_ac0_73`] module*/
pub type HUFFENC_AC0_73 = crate::Reg<huffenc_ac0_73::HUFFENC_AC0_73rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_73;
/**HUFFENC_AC0_74 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_74)

For information about available fields see [`mod@huffenc_ac0_74`] module*/
pub type HUFFENC_AC0_74 = crate::Reg<huffenc_ac0_74::HUFFENC_AC0_74rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_74;
/**HUFFENC_AC0_75 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_75)

For information about available fields see [`mod@huffenc_ac0_75`] module*/
pub type HUFFENC_AC0_75 = crate::Reg<huffenc_ac0_75::HUFFENC_AC0_75rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_75;
/**HUFFENC_AC0_76 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_76)

For information about available fields see [`mod@huffenc_ac0_76`] module*/
pub type HUFFENC_AC0_76 = crate::Reg<huffenc_ac0_76::HUFFENC_AC0_76rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_76;
/**HUFFENC_AC0_77 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_77)

For information about available fields see [`mod@huffenc_ac0_77`] module*/
pub type HUFFENC_AC0_77 = crate::Reg<huffenc_ac0_77::HUFFENC_AC0_77rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_77;
/**HUFFENC_AC0_78 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_78)

For information about available fields see [`mod@huffenc_ac0_78`] module*/
pub type HUFFENC_AC0_78 = crate::Reg<huffenc_ac0_78::HUFFENC_AC0_78rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_78;
/**HUFFENC_AC0_79 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_79)

For information about available fields see [`mod@huffenc_ac0_79`] module*/
pub type HUFFENC_AC0_79 = crate::Reg<huffenc_ac0_79::HUFFENC_AC0_79rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_79;
/**HUFFENC_AC0_80 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_80)

For information about available fields see [`mod@huffenc_ac0_80`] module*/
pub type HUFFENC_AC0_80 = crate::Reg<huffenc_ac0_80::HUFFENC_AC0_80rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_80;
/**HUFFENC_AC0_81 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_81)

For information about available fields see [`mod@huffenc_ac0_81`] module*/
pub type HUFFENC_AC0_81 = crate::Reg<huffenc_ac0_81::HUFFENC_AC0_81rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_81;
/**HUFFENC_AC0_82 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_82)

For information about available fields see [`mod@huffenc_ac0_82`] module*/
pub type HUFFENC_AC0_82 = crate::Reg<huffenc_ac0_82::HUFFENC_AC0_82rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_82;
/**HUFFENC_AC0_83 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_83)

For information about available fields see [`mod@huffenc_ac0_83`] module*/
pub type HUFFENC_AC0_83 = crate::Reg<huffenc_ac0_83::HUFFENC_AC0_83rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_83;
/**HUFFENC_AC0_84 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_84)

For information about available fields see [`mod@huffenc_ac0_84`] module*/
pub type HUFFENC_AC0_84 = crate::Reg<huffenc_ac0_84::HUFFENC_AC0_84rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_84;
/**HUFFENC_AC0_85 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_85)

For information about available fields see [`mod@huffenc_ac0_85`] module*/
pub type HUFFENC_AC0_85 = crate::Reg<huffenc_ac0_85::HUFFENC_AC0_85rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_85;
/**HUFFENC_AC0_86 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_86)

For information about available fields see [`mod@huffenc_ac0_86`] module*/
pub type HUFFENC_AC0_86 = crate::Reg<huffenc_ac0_86::HUFFENC_AC0_86rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_86;
/**HUFFENC_AC0_87 (rw) register accessor: JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_87)

For information about available fields see [`mod@huffenc_ac0_87`] module*/
pub type HUFFENC_AC0_87 = crate::Reg<huffenc_ac0_87::HUFFENC_AC0_87rs>;
///JPEG Huffman encoder AC0
pub mod huffenc_ac0_87;
/**HUFFENC_AC1_0 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_0)

For information about available fields see [`mod@huffenc_ac1_0`] module*/
pub type HUFFENC_AC1_0 = crate::Reg<huffenc_ac1_0::HUFFENC_AC1_0rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_0;
/**HUFFENC_AC1_1 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_1)

For information about available fields see [`mod@huffenc_ac1_1`] module*/
pub type HUFFENC_AC1_1 = crate::Reg<huffenc_ac1_1::HUFFENC_AC1_1rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_1;
/**HUFFENC_AC1_2 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_2)

For information about available fields see [`mod@huffenc_ac1_2`] module*/
pub type HUFFENC_AC1_2 = crate::Reg<huffenc_ac1_2::HUFFENC_AC1_2rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_2;
/**HUFFENC_AC1_3 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_3)

For information about available fields see [`mod@huffenc_ac1_3`] module*/
pub type HUFFENC_AC1_3 = crate::Reg<huffenc_ac1_3::HUFFENC_AC1_3rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_3;
/**HUFFENC_AC1_4 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_4)

For information about available fields see [`mod@huffenc_ac1_4`] module*/
pub type HUFFENC_AC1_4 = crate::Reg<huffenc_ac1_4::HUFFENC_AC1_4rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_4;
/**HUFFENC_AC1_5 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_5)

For information about available fields see [`mod@huffenc_ac1_5`] module*/
pub type HUFFENC_AC1_5 = crate::Reg<huffenc_ac1_5::HUFFENC_AC1_5rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_5;
/**HUFFENC_AC1_6 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_6)

For information about available fields see [`mod@huffenc_ac1_6`] module*/
pub type HUFFENC_AC1_6 = crate::Reg<huffenc_ac1_6::HUFFENC_AC1_6rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_6;
/**HUFFENC_AC1_7 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_7)

For information about available fields see [`mod@huffenc_ac1_7`] module*/
pub type HUFFENC_AC1_7 = crate::Reg<huffenc_ac1_7::HUFFENC_AC1_7rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_7;
/**HUFFENC_AC1_8 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_8)

For information about available fields see [`mod@huffenc_ac1_8`] module*/
pub type HUFFENC_AC1_8 = crate::Reg<huffenc_ac1_8::HUFFENC_AC1_8rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_8;
/**HUFFENC_AC1_9 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_9)

For information about available fields see [`mod@huffenc_ac1_9`] module*/
pub type HUFFENC_AC1_9 = crate::Reg<huffenc_ac1_9::HUFFENC_AC1_9rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_9;
/**HUFFENC_AC1_10 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_10)

For information about available fields see [`mod@huffenc_ac1_10`] module*/
pub type HUFFENC_AC1_10 = crate::Reg<huffenc_ac1_10::HUFFENC_AC1_10rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_10;
/**HUFFENC_AC1_11 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_11)

For information about available fields see [`mod@huffenc_ac1_11`] module*/
pub type HUFFENC_AC1_11 = crate::Reg<huffenc_ac1_11::HUFFENC_AC1_11rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_11;
/**HUFFENC_AC1_12 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_12)

For information about available fields see [`mod@huffenc_ac1_12`] module*/
pub type HUFFENC_AC1_12 = crate::Reg<huffenc_ac1_12::HUFFENC_AC1_12rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_12;
/**HUFFENC_AC1_13 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_13)

For information about available fields see [`mod@huffenc_ac1_13`] module*/
pub type HUFFENC_AC1_13 = crate::Reg<huffenc_ac1_13::HUFFENC_AC1_13rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_13;
/**HUFFENC_AC1_14 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_14)

For information about available fields see [`mod@huffenc_ac1_14`] module*/
pub type HUFFENC_AC1_14 = crate::Reg<huffenc_ac1_14::HUFFENC_AC1_14rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_14;
/**HUFFENC_AC1_15 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_15)

For information about available fields see [`mod@huffenc_ac1_15`] module*/
pub type HUFFENC_AC1_15 = crate::Reg<huffenc_ac1_15::HUFFENC_AC1_15rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_15;
/**HUFFENC_AC1_16 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_16)

For information about available fields see [`mod@huffenc_ac1_16`] module*/
pub type HUFFENC_AC1_16 = crate::Reg<huffenc_ac1_16::HUFFENC_AC1_16rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_16;
/**HUFFENC_AC1_17 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_17)

For information about available fields see [`mod@huffenc_ac1_17`] module*/
pub type HUFFENC_AC1_17 = crate::Reg<huffenc_ac1_17::HUFFENC_AC1_17rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_17;
/**HUFFENC_AC1_18 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_18)

For information about available fields see [`mod@huffenc_ac1_18`] module*/
pub type HUFFENC_AC1_18 = crate::Reg<huffenc_ac1_18::HUFFENC_AC1_18rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_18;
/**HUFFENC_AC1_19 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_19)

For information about available fields see [`mod@huffenc_ac1_19`] module*/
pub type HUFFENC_AC1_19 = crate::Reg<huffenc_ac1_19::HUFFENC_AC1_19rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_19;
/**HUFFENC_AC1_20 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_20)

For information about available fields see [`mod@huffenc_ac1_20`] module*/
pub type HUFFENC_AC1_20 = crate::Reg<huffenc_ac1_20::HUFFENC_AC1_20rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_20;
/**HUFFENC_AC1_21 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_21)

For information about available fields see [`mod@huffenc_ac1_21`] module*/
pub type HUFFENC_AC1_21 = crate::Reg<huffenc_ac1_21::HUFFENC_AC1_21rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_21;
/**HUFFENC_AC1_22 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_22)

For information about available fields see [`mod@huffenc_ac1_22`] module*/
pub type HUFFENC_AC1_22 = crate::Reg<huffenc_ac1_22::HUFFENC_AC1_22rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_22;
/**HUFFENC_AC1_23 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_23)

For information about available fields see [`mod@huffenc_ac1_23`] module*/
pub type HUFFENC_AC1_23 = crate::Reg<huffenc_ac1_23::HUFFENC_AC1_23rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_23;
/**HUFFENC_AC1_24 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_24)

For information about available fields see [`mod@huffenc_ac1_24`] module*/
pub type HUFFENC_AC1_24 = crate::Reg<huffenc_ac1_24::HUFFENC_AC1_24rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_24;
/**HUFFENC_AC1_25 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_25)

For information about available fields see [`mod@huffenc_ac1_25`] module*/
pub type HUFFENC_AC1_25 = crate::Reg<huffenc_ac1_25::HUFFENC_AC1_25rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_25;
/**HUFFENC_AC1_26 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_26)

For information about available fields see [`mod@huffenc_ac1_26`] module*/
pub type HUFFENC_AC1_26 = crate::Reg<huffenc_ac1_26::HUFFENC_AC1_26rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_26;
/**HUFFENC_AC1_27 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_27)

For information about available fields see [`mod@huffenc_ac1_27`] module*/
pub type HUFFENC_AC1_27 = crate::Reg<huffenc_ac1_27::HUFFENC_AC1_27rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_27;
/**HUFFENC_AC1_28 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_28)

For information about available fields see [`mod@huffenc_ac1_28`] module*/
pub type HUFFENC_AC1_28 = crate::Reg<huffenc_ac1_28::HUFFENC_AC1_28rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_28;
/**HUFFENC_AC1_29 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_29)

For information about available fields see [`mod@huffenc_ac1_29`] module*/
pub type HUFFENC_AC1_29 = crate::Reg<huffenc_ac1_29::HUFFENC_AC1_29rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_29;
/**HUFFENC_AC1_30 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_30)

For information about available fields see [`mod@huffenc_ac1_30`] module*/
pub type HUFFENC_AC1_30 = crate::Reg<huffenc_ac1_30::HUFFENC_AC1_30rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_30;
/**HUFFENC_AC1_31 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_31)

For information about available fields see [`mod@huffenc_ac1_31`] module*/
pub type HUFFENC_AC1_31 = crate::Reg<huffenc_ac1_31::HUFFENC_AC1_31rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_31;
/**HUFFENC_AC1_32 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_32)

For information about available fields see [`mod@huffenc_ac1_32`] module*/
pub type HUFFENC_AC1_32 = crate::Reg<huffenc_ac1_32::HUFFENC_AC1_32rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_32;
/**HUFFENC_AC1_33 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_33)

For information about available fields see [`mod@huffenc_ac1_33`] module*/
pub type HUFFENC_AC1_33 = crate::Reg<huffenc_ac1_33::HUFFENC_AC1_33rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_33;
/**HUFFENC_AC1_34 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_34)

For information about available fields see [`mod@huffenc_ac1_34`] module*/
pub type HUFFENC_AC1_34 = crate::Reg<huffenc_ac1_34::HUFFENC_AC1_34rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_34;
/**HUFFENC_AC1_35 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_35)

For information about available fields see [`mod@huffenc_ac1_35`] module*/
pub type HUFFENC_AC1_35 = crate::Reg<huffenc_ac1_35::HUFFENC_AC1_35rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_35;
/**HUFFENC_AC1_36 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_36)

For information about available fields see [`mod@huffenc_ac1_36`] module*/
pub type HUFFENC_AC1_36 = crate::Reg<huffenc_ac1_36::HUFFENC_AC1_36rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_36;
/**HUFFENC_AC1_37 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_37)

For information about available fields see [`mod@huffenc_ac1_37`] module*/
pub type HUFFENC_AC1_37 = crate::Reg<huffenc_ac1_37::HUFFENC_AC1_37rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_37;
/**HUFFENC_AC1_38 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_38)

For information about available fields see [`mod@huffenc_ac1_38`] module*/
pub type HUFFENC_AC1_38 = crate::Reg<huffenc_ac1_38::HUFFENC_AC1_38rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_38;
/**HUFFENC_AC1_39 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_39)

For information about available fields see [`mod@huffenc_ac1_39`] module*/
pub type HUFFENC_AC1_39 = crate::Reg<huffenc_ac1_39::HUFFENC_AC1_39rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_39;
/**HUFFENC_AC1_40 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_40)

For information about available fields see [`mod@huffenc_ac1_40`] module*/
pub type HUFFENC_AC1_40 = crate::Reg<huffenc_ac1_40::HUFFENC_AC1_40rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_40;
/**HUFFENC_AC1_41 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_41)

For information about available fields see [`mod@huffenc_ac1_41`] module*/
pub type HUFFENC_AC1_41 = crate::Reg<huffenc_ac1_41::HUFFENC_AC1_41rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_41;
/**HUFFENC_AC1_42 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_42)

For information about available fields see [`mod@huffenc_ac1_42`] module*/
pub type HUFFENC_AC1_42 = crate::Reg<huffenc_ac1_42::HUFFENC_AC1_42rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_42;
/**HUFFENC_AC1_43 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_43)

For information about available fields see [`mod@huffenc_ac1_43`] module*/
pub type HUFFENC_AC1_43 = crate::Reg<huffenc_ac1_43::HUFFENC_AC1_43rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_43;
/**HUFFENC_AC1_44 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_44)

For information about available fields see [`mod@huffenc_ac1_44`] module*/
pub type HUFFENC_AC1_44 = crate::Reg<huffenc_ac1_44::HUFFENC_AC1_44rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_44;
/**HUFFENC_AC1_45 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_45)

For information about available fields see [`mod@huffenc_ac1_45`] module*/
pub type HUFFENC_AC1_45 = crate::Reg<huffenc_ac1_45::HUFFENC_AC1_45rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_45;
/**HUFFENC_AC1_46 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_46)

For information about available fields see [`mod@huffenc_ac1_46`] module*/
pub type HUFFENC_AC1_46 = crate::Reg<huffenc_ac1_46::HUFFENC_AC1_46rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_46;
/**HUFFENC_AC1_47 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_47)

For information about available fields see [`mod@huffenc_ac1_47`] module*/
pub type HUFFENC_AC1_47 = crate::Reg<huffenc_ac1_47::HUFFENC_AC1_47rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_47;
/**HUFFENC_AC1_48 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_48)

For information about available fields see [`mod@huffenc_ac1_48`] module*/
pub type HUFFENC_AC1_48 = crate::Reg<huffenc_ac1_48::HUFFENC_AC1_48rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_48;
/**HUFFENC_AC1_49 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_49)

For information about available fields see [`mod@huffenc_ac1_49`] module*/
pub type HUFFENC_AC1_49 = crate::Reg<huffenc_ac1_49::HUFFENC_AC1_49rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_49;
/**HUFFENC_AC1_50 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_50)

For information about available fields see [`mod@huffenc_ac1_50`] module*/
pub type HUFFENC_AC1_50 = crate::Reg<huffenc_ac1_50::HUFFENC_AC1_50rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_50;
/**HUFFENC_AC1_51 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_51)

For information about available fields see [`mod@huffenc_ac1_51`] module*/
pub type HUFFENC_AC1_51 = crate::Reg<huffenc_ac1_51::HUFFENC_AC1_51rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_51;
/**HUFFENC_AC1_52 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_52)

For information about available fields see [`mod@huffenc_ac1_52`] module*/
pub type HUFFENC_AC1_52 = crate::Reg<huffenc_ac1_52::HUFFENC_AC1_52rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_52;
/**HUFFENC_AC1_53 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_53)

For information about available fields see [`mod@huffenc_ac1_53`] module*/
pub type HUFFENC_AC1_53 = crate::Reg<huffenc_ac1_53::HUFFENC_AC1_53rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_53;
/**HUFFENC_AC1_54 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_54)

For information about available fields see [`mod@huffenc_ac1_54`] module*/
pub type HUFFENC_AC1_54 = crate::Reg<huffenc_ac1_54::HUFFENC_AC1_54rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_54;
/**HUFFENC_AC1_55 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_55)

For information about available fields see [`mod@huffenc_ac1_55`] module*/
pub type HUFFENC_AC1_55 = crate::Reg<huffenc_ac1_55::HUFFENC_AC1_55rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_55;
/**HUFFENC_AC1_56 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_56)

For information about available fields see [`mod@huffenc_ac1_56`] module*/
pub type HUFFENC_AC1_56 = crate::Reg<huffenc_ac1_56::HUFFENC_AC1_56rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_56;
/**HUFFENC_AC1_57 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_57)

For information about available fields see [`mod@huffenc_ac1_57`] module*/
pub type HUFFENC_AC1_57 = crate::Reg<huffenc_ac1_57::HUFFENC_AC1_57rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_57;
/**HUFFENC_AC1_58 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_58)

For information about available fields see [`mod@huffenc_ac1_58`] module*/
pub type HUFFENC_AC1_58 = crate::Reg<huffenc_ac1_58::HUFFENC_AC1_58rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_58;
/**HUFFENC_AC1_59 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_59)

For information about available fields see [`mod@huffenc_ac1_59`] module*/
pub type HUFFENC_AC1_59 = crate::Reg<huffenc_ac1_59::HUFFENC_AC1_59rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_59;
/**HUFFENC_AC1_60 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_60)

For information about available fields see [`mod@huffenc_ac1_60`] module*/
pub type HUFFENC_AC1_60 = crate::Reg<huffenc_ac1_60::HUFFENC_AC1_60rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_60;
/**HUFFENC_AC1_61 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_61)

For information about available fields see [`mod@huffenc_ac1_61`] module*/
pub type HUFFENC_AC1_61 = crate::Reg<huffenc_ac1_61::HUFFENC_AC1_61rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_61;
/**HUFFENC_AC1_62 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_62)

For information about available fields see [`mod@huffenc_ac1_62`] module*/
pub type HUFFENC_AC1_62 = crate::Reg<huffenc_ac1_62::HUFFENC_AC1_62rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_62;
/**HUFFENC_AC1_63 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_63)

For information about available fields see [`mod@huffenc_ac1_63`] module*/
pub type HUFFENC_AC1_63 = crate::Reg<huffenc_ac1_63::HUFFENC_AC1_63rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_63;
/**HUFFENC_AC1_64 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_64)

For information about available fields see [`mod@huffenc_ac1_64`] module*/
pub type HUFFENC_AC1_64 = crate::Reg<huffenc_ac1_64::HUFFENC_AC1_64rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_64;
/**HUFFENC_AC1_65 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_65)

For information about available fields see [`mod@huffenc_ac1_65`] module*/
pub type HUFFENC_AC1_65 = crate::Reg<huffenc_ac1_65::HUFFENC_AC1_65rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_65;
/**HUFFENC_AC1_66 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_66)

For information about available fields see [`mod@huffenc_ac1_66`] module*/
pub type HUFFENC_AC1_66 = crate::Reg<huffenc_ac1_66::HUFFENC_AC1_66rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_66;
/**HUFFENC_AC1_67 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_67)

For information about available fields see [`mod@huffenc_ac1_67`] module*/
pub type HUFFENC_AC1_67 = crate::Reg<huffenc_ac1_67::HUFFENC_AC1_67rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_67;
/**HUFFENC_AC1_68 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_68)

For information about available fields see [`mod@huffenc_ac1_68`] module*/
pub type HUFFENC_AC1_68 = crate::Reg<huffenc_ac1_68::HUFFENC_AC1_68rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_68;
/**HUFFENC_AC1_69 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_69)

For information about available fields see [`mod@huffenc_ac1_69`] module*/
pub type HUFFENC_AC1_69 = crate::Reg<huffenc_ac1_69::HUFFENC_AC1_69rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_69;
/**HUFFENC_AC1_70 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_70)

For information about available fields see [`mod@huffenc_ac1_70`] module*/
pub type HUFFENC_AC1_70 = crate::Reg<huffenc_ac1_70::HUFFENC_AC1_70rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_70;
/**HUFFENC_AC1_71 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_71)

For information about available fields see [`mod@huffenc_ac1_71`] module*/
pub type HUFFENC_AC1_71 = crate::Reg<huffenc_ac1_71::HUFFENC_AC1_71rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_71;
/**HUFFENC_AC1_72 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_72)

For information about available fields see [`mod@huffenc_ac1_72`] module*/
pub type HUFFENC_AC1_72 = crate::Reg<huffenc_ac1_72::HUFFENC_AC1_72rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_72;
/**HUFFENC_AC1_73 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_73)

For information about available fields see [`mod@huffenc_ac1_73`] module*/
pub type HUFFENC_AC1_73 = crate::Reg<huffenc_ac1_73::HUFFENC_AC1_73rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_73;
/**HUFFENC_AC1_74 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_74)

For information about available fields see [`mod@huffenc_ac1_74`] module*/
pub type HUFFENC_AC1_74 = crate::Reg<huffenc_ac1_74::HUFFENC_AC1_74rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_74;
/**HUFFENC_AC1_75 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_75)

For information about available fields see [`mod@huffenc_ac1_75`] module*/
pub type HUFFENC_AC1_75 = crate::Reg<huffenc_ac1_75::HUFFENC_AC1_75rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_75;
/**HUFFENC_AC1_76 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_76)

For information about available fields see [`mod@huffenc_ac1_76`] module*/
pub type HUFFENC_AC1_76 = crate::Reg<huffenc_ac1_76::HUFFENC_AC1_76rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_76;
/**HUFFENC_AC1_77 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_77)

For information about available fields see [`mod@huffenc_ac1_77`] module*/
pub type HUFFENC_AC1_77 = crate::Reg<huffenc_ac1_77::HUFFENC_AC1_77rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_77;
/**HUFFENC_AC1_78 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_78)

For information about available fields see [`mod@huffenc_ac1_78`] module*/
pub type HUFFENC_AC1_78 = crate::Reg<huffenc_ac1_78::HUFFENC_AC1_78rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_78;
/**HUFFENC_AC1_79 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_79)

For information about available fields see [`mod@huffenc_ac1_79`] module*/
pub type HUFFENC_AC1_79 = crate::Reg<huffenc_ac1_79::HUFFENC_AC1_79rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_79;
/**HUFFENC_AC1_80 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_80)

For information about available fields see [`mod@huffenc_ac1_80`] module*/
pub type HUFFENC_AC1_80 = crate::Reg<huffenc_ac1_80::HUFFENC_AC1_80rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_80;
/**HUFFENC_AC1_81 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_81)

For information about available fields see [`mod@huffenc_ac1_81`] module*/
pub type HUFFENC_AC1_81 = crate::Reg<huffenc_ac1_81::HUFFENC_AC1_81rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_81;
/**HUFFENC_AC1_82 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_82)

For information about available fields see [`mod@huffenc_ac1_82`] module*/
pub type HUFFENC_AC1_82 = crate::Reg<huffenc_ac1_82::HUFFENC_AC1_82rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_82;
/**HUFFENC_AC1_83 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_83)

For information about available fields see [`mod@huffenc_ac1_83`] module*/
pub type HUFFENC_AC1_83 = crate::Reg<huffenc_ac1_83::HUFFENC_AC1_83rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_83;
/**HUFFENC_AC1_84 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_84)

For information about available fields see [`mod@huffenc_ac1_84`] module*/
pub type HUFFENC_AC1_84 = crate::Reg<huffenc_ac1_84::HUFFENC_AC1_84rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_84;
/**HUFFENC_AC1_85 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_85)

For information about available fields see [`mod@huffenc_ac1_85`] module*/
pub type HUFFENC_AC1_85 = crate::Reg<huffenc_ac1_85::HUFFENC_AC1_85rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_85;
/**HUFFENC_AC1_86 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_86)

For information about available fields see [`mod@huffenc_ac1_86`] module*/
pub type HUFFENC_AC1_86 = crate::Reg<huffenc_ac1_86::HUFFENC_AC1_86rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_86;
/**HUFFENC_AC1_87 (rw) register accessor: JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_87)

For information about available fields see [`mod@huffenc_ac1_87`] module*/
pub type HUFFENC_AC1_87 = crate::Reg<huffenc_ac1_87::HUFFENC_AC1_87rs>;
///JPEG Huffman encoder AC1
pub mod huffenc_ac1_87;
/**HUFFENC_DC0_0 (rw) register accessor: JPEG Huffman encoder DC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC0_0)

For information about available fields see [`mod@huffenc_dc0_0`] module*/
pub type HUFFENC_DC0_0 = crate::Reg<huffenc_dc0_0::HUFFENC_DC0_0rs>;
///JPEG Huffman encoder DC0
pub mod huffenc_dc0_0;
/**HUFFENC_DC0_1 (rw) register accessor: JPEG Huffman encoder DC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC0_1)

For information about available fields see [`mod@huffenc_dc0_1`] module*/
pub type HUFFENC_DC0_1 = crate::Reg<huffenc_dc0_1::HUFFENC_DC0_1rs>;
///JPEG Huffman encoder DC0
pub mod huffenc_dc0_1;
/**HUFFENC_DC0_2 (rw) register accessor: JPEG Huffman encoder DC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC0_2)

For information about available fields see [`mod@huffenc_dc0_2`] module*/
pub type HUFFENC_DC0_2 = crate::Reg<huffenc_dc0_2::HUFFENC_DC0_2rs>;
///JPEG Huffman encoder DC0
pub mod huffenc_dc0_2;
/**HUFFENC_DC0_3 (rw) register accessor: JPEG Huffman encoder DC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC0_3)

For information about available fields see [`mod@huffenc_dc0_3`] module*/
pub type HUFFENC_DC0_3 = crate::Reg<huffenc_dc0_3::HUFFENC_DC0_3rs>;
///JPEG Huffman encoder DC0
pub mod huffenc_dc0_3;
/**HUFFENC_DC0_4 (rw) register accessor: JPEG Huffman encoder DC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC0_4)

For information about available fields see [`mod@huffenc_dc0_4`] module*/
pub type HUFFENC_DC0_4 = crate::Reg<huffenc_dc0_4::HUFFENC_DC0_4rs>;
///JPEG Huffman encoder DC0
pub mod huffenc_dc0_4;
/**HUFFENC_DC0_5 (rw) register accessor: JPEG Huffman encoder DC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC0_5)

For information about available fields see [`mod@huffenc_dc0_5`] module*/
pub type HUFFENC_DC0_5 = crate::Reg<huffenc_dc0_5::HUFFENC_DC0_5rs>;
///JPEG Huffman encoder DC0
pub mod huffenc_dc0_5;
/**HUFFENC_DC0_6 (rw) register accessor: JPEG Huffman encoder DC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC0_6)

For information about available fields see [`mod@huffenc_dc0_6`] module*/
pub type HUFFENC_DC0_6 = crate::Reg<huffenc_dc0_6::HUFFENC_DC0_6rs>;
///JPEG Huffman encoder DC0
pub mod huffenc_dc0_6;
/**HUFFENC_DC0_7 (rw) register accessor: JPEG Huffman encoder DC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC0_7)

For information about available fields see [`mod@huffenc_dc0_7`] module*/
pub type HUFFENC_DC0_7 = crate::Reg<huffenc_dc0_7::HUFFENC_DC0_7rs>;
///JPEG Huffman encoder DC0
pub mod huffenc_dc0_7;
/**HUFFENC_DC1_0 (rw) register accessor: JPEG Huffman encoder DC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC1_0)

For information about available fields see [`mod@huffenc_dc1_0`] module*/
pub type HUFFENC_DC1_0 = crate::Reg<huffenc_dc1_0::HUFFENC_DC1_0rs>;
///JPEG Huffman encoder DC1
pub mod huffenc_dc1_0;
/**HUFFENC_DC1_1 (rw) register accessor: JPEG Huffman encoder DC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC1_1)

For information about available fields see [`mod@huffenc_dc1_1`] module*/
pub type HUFFENC_DC1_1 = crate::Reg<huffenc_dc1_1::HUFFENC_DC1_1rs>;
///JPEG Huffman encoder DC1
pub mod huffenc_dc1_1;
/**HUFFENC_DC1_2 (rw) register accessor: JPEG Huffman encoder DC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC1_2)

For information about available fields see [`mod@huffenc_dc1_2`] module*/
pub type HUFFENC_DC1_2 = crate::Reg<huffenc_dc1_2::HUFFENC_DC1_2rs>;
///JPEG Huffman encoder DC1
pub mod huffenc_dc1_2;
/**HUFFENC_DC1_3 (rw) register accessor: JPEG Huffman encoder DC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC1_3)

For information about available fields see [`mod@huffenc_dc1_3`] module*/
pub type HUFFENC_DC1_3 = crate::Reg<huffenc_dc1_3::HUFFENC_DC1_3rs>;
///JPEG Huffman encoder DC1
pub mod huffenc_dc1_3;
/**HUFFENC_DC1_4 (rw) register accessor: JPEG Huffman encoder DC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC1_4)

For information about available fields see [`mod@huffenc_dc1_4`] module*/
pub type HUFFENC_DC1_4 = crate::Reg<huffenc_dc1_4::HUFFENC_DC1_4rs>;
///JPEG Huffman encoder DC1
pub mod huffenc_dc1_4;
/**HUFFENC_DC1_5 (rw) register accessor: JPEG Huffman encoder DC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC1_5)

For information about available fields see [`mod@huffenc_dc1_5`] module*/
pub type HUFFENC_DC1_5 = crate::Reg<huffenc_dc1_5::HUFFENC_DC1_5rs>;
///JPEG Huffman encoder DC1
pub mod huffenc_dc1_5;
/**HUFFENC_DC1_6 (rw) register accessor: JPEG Huffman encoder DC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC1_6)

For information about available fields see [`mod@huffenc_dc1_6`] module*/
pub type HUFFENC_DC1_6 = crate::Reg<huffenc_dc1_6::HUFFENC_DC1_6rs>;
///JPEG Huffman encoder DC1
pub mod huffenc_dc1_6;
/**HUFFENC_DC1_7 (rw) register accessor: JPEG Huffman encoder DC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC1_7)

For information about available fields see [`mod@huffenc_dc1_7`] module*/
pub type HUFFENC_DC1_7 = crate::Reg<huffenc_dc1_7::HUFFENC_DC1_7rs>;
///JPEG Huffman encoder DC1
pub mod huffenc_dc1_7;
