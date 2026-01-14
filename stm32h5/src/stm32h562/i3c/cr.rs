///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DCNT` writer - count of data to transfer during a read or write message, in bytes (whatever I3C is acting as controller/target) Linear encoding up to 64 Kbytes -1 ...
pub type DCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RNW` writer - read / non-write message (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\[3:0\]=0010 (private message) or MTYPE\[3:0\]=0011 (direct message) or MTYPE\[3:0\]=0100 (legacy I2C message), in order to emit the RnW bit on the I3C bus.
pub type RNW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD` writer - 7-bit I3C dynamic / I2C static target address (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\[3:0\]=0010 (private message) or MTYPE\[3:0\]=0011 (direct message) or MTYPE\[3:0\]=0100 (legacy I2C message)
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `MTYPE` writer - message type (whatever I3C is acting as controller/target) Bits\[26:0\] are ignored. After M2 error detection on an I3C SDR message, this is needed for SCL 'stuck at' recovery. Bits\[26:0\] are ignored. If I3C_CFGR.EXITPTRN=1, an HDR exit pattern is emitted on the bus to generate an escalation fault. Bits\[23:17\] (ADD\[6:0\]) is the emitted 7-bit dynamic address. Bit\[16\] (RNW) is the emitted RnW bit. The transferred private message is: {S / S+7'h7E+RnW=0+Sr / Sr+*} + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). Bits\[23:17\] (ADD\[6:0\]) is the emitted 7-bit dynamic address. Bit\[16\] (RNW) is the emitted RnW bit. The transferred direct message is: Sr + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P Bits\[23:17\] (ADD\[6:0\]) is the emitted 7-bit static address. Bit\[16\] (RNW) is the emitted RnW bit. The transferred legacy I2C message is: {S / S+ 7'h7E+RnW=0 + Sr / Sr+*} + 7-bit StaAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). 1xxx: reserved (when I3C is acting as I3C controller, used when target) 0xxx: reserved {S +} 7'h02 addr + RnW=0 {S +} 7-bit I3C_DEVR0.DA\[6:0\] + RnW=0 after a bus available condition (the target first emits a START request), or once the controller drives a START. {S +} 7-bit I3C_DEVR0.DA\[6:0\] + RnW=1 (+Ack/Nack from controller) When acknowledged from controller, the next (optional, depending on I3C_BCR.BCR2) transmitted IBI payload data is defined by I3C_CR.DCNT\[15:0\] and must be consistently programmed vs the maximum IBI payload data size which is defined by I3C_IBIDR.IBIP\[2:0\]. Others: reserved
pub type MTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MEND` writer - message end type (when the I3C is acting as controller)
pub type MEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - count of data to transfer during a read or write message, in bytes (whatever I3C is acting as controller/target) Linear encoding up to 64 Kbytes -1 ...
    #[inline(always)]
    pub fn dcnt(&mut self) -> DCNT_W<'_, CRrs> {
        DCNT_W::new(self, 0)
    }
    ///Bit 16 - read / non-write message (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\[3:0\]=0010 (private message) or MTYPE\[3:0\]=0011 (direct message) or MTYPE\[3:0\]=0100 (legacy I2C message), in order to emit the RnW bit on the I3C bus.
    #[inline(always)]
    pub fn rnw(&mut self) -> RNW_W<'_, CRrs> {
        RNW_W::new(self, 16)
    }
    ///Bits 17:23 - 7-bit I3C dynamic / I2C static target address (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\[3:0\]=0010 (private message) or MTYPE\[3:0\]=0011 (direct message) or MTYPE\[3:0\]=0100 (legacy I2C message)
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, CRrs> {
        ADD_W::new(self, 17)
    }
    ///Bits 27:30 - message type (whatever I3C is acting as controller/target) Bits\[26:0\] are ignored. After M2 error detection on an I3C SDR message, this is needed for SCL 'stuck at' recovery. Bits\[26:0\] are ignored. If I3C_CFGR.EXITPTRN=1, an HDR exit pattern is emitted on the bus to generate an escalation fault. Bits\[23:17\] (ADD\[6:0\]) is the emitted 7-bit dynamic address. Bit\[16\] (RNW) is the emitted RnW bit. The transferred private message is: {S / S+7'h7E+RnW=0+Sr / Sr+*} + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). Bits\[23:17\] (ADD\[6:0\]) is the emitted 7-bit dynamic address. Bit\[16\] (RNW) is the emitted RnW bit. The transferred direct message is: Sr + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P Bits\[23:17\] (ADD\[6:0\]) is the emitted 7-bit static address. Bit\[16\] (RNW) is the emitted RnW bit. The transferred legacy I2C message is: {S / S+ 7'h7E+RnW=0 + Sr / Sr+*} + 7-bit StaAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). 1xxx: reserved (when I3C is acting as I3C controller, used when target) 0xxx: reserved {S +} 7'h02 addr + RnW=0 {S +} 7-bit I3C_DEVR0.DA\[6:0\] + RnW=0 after a bus available condition (the target first emits a START request), or once the controller drives a START. {S +} 7-bit I3C_DEVR0.DA\[6:0\] + RnW=1 (+Ack/Nack from controller) When acknowledged from controller, the next (optional, depending on I3C_BCR.BCR2) transmitted IBI payload data is defined by I3C_CR.DCNT\[15:0\] and must be consistently programmed vs the maximum IBI payload data size which is defined by I3C_IBIDR.IBIP\[2:0\]. Others: reserved
    #[inline(always)]
    pub fn mtype(&mut self) -> MTYPE_W<'_, CRrs> {
        MTYPE_W::new(self, 27)
    }
    ///Bit 31 - message end type (when the I3C is acting as controller)
    #[inline(always)]
    pub fn mend(&mut self) -> MEND_W<'_, CRrs> {
        MEND_W::new(self, 31)
    }
}
/**I3C message control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#I3C:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
