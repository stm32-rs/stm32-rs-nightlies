///Register `I3C_CR_ALTERNATE` writer
pub type W = crate::W<I3C_CR_ALTERNATErs>;
///Field `DCNT` writer - count of data to transfer during a read or write message, in bytes (when I3C is acting as controller) Linear encoding up to 64 Kbytes -1. ...
pub type DCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CCC` writer - 8-bit CCC code (when I3C is acting as controller) If Bit\[23\]=CCC\[7\]=1, this is the 1st part of an I3C SDR direct CCC command. If Bit\[23\]=CCC\[7\]=0, this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0).
pub type CCC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**Field `MTYPE` writer - message type (when I3C is acting as controller) Bits\[23:16\]
(CCC\[7:0\]) is the emitted 8-bit CCC code If Bit\[23\]=CCC\[7\]=1: this is the 1st part of an I3C SDR direct CCC command The transferred direct CCC command message is: {S / S+7'h7E +RnW=0 / Sr+*} + (direct CCC + T) + (8-bit Data + T)* + Sr After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+R/W). If Bit\[23\]=CCC\[7\]=0: this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0) The transferred broadcast CCC command message is: {S / S+7'h7E +RnW=0 / Sr+*} + (broadcast CCC + T) + (8-bit Data + T)* + Sr/P After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+R/W). others: reserved*/
pub type MTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MEND` writer - message end type (when I3C is acting as controller)
pub type MEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<I3C_CR_ALTERNATErs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - count of data to transfer during a read or write message, in bytes (when I3C is acting as controller) Linear encoding up to 64 Kbytes -1. ...
    #[inline(always)]
    pub fn dcnt(&mut self) -> DCNT_W<I3C_CR_ALTERNATErs> {
        DCNT_W::new(self, 0)
    }
    ///Bits 16:23 - 8-bit CCC code (when I3C is acting as controller) If Bit\[23\]=CCC\[7\]=1, this is the 1st part of an I3C SDR direct CCC command. If Bit\[23\]=CCC\[7\]=0, this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0).
    #[inline(always)]
    pub fn ccc(&mut self) -> CCC_W<I3C_CR_ALTERNATErs> {
        CCC_W::new(self, 16)
    }
    /**Bits 27:30 - message type (when I3C is acting as controller) Bits\[23:16\]
    (CCC\[7:0\]) is the emitted 8-bit CCC code If Bit\[23\]=CCC\[7\]=1: this is the 1st part of an I3C SDR direct CCC command The transferred direct CCC command message is: {S / S+7'h7E +RnW=0 / Sr+*} + (direct CCC + T) + (8-bit Data + T)* + Sr After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+R/W). If Bit\[23\]=CCC\[7\]=0: this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0) The transferred broadcast CCC command message is: {S / S+7'h7E +RnW=0 / Sr+*} + (broadcast CCC + T) + (8-bit Data + T)* + Sr/P After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+R/W). others: reserved*/
    #[inline(always)]
    pub fn mtype(&mut self) -> MTYPE_W<I3C_CR_ALTERNATErs> {
        MTYPE_W::new(self, 27)
    }
    ///Bit 31 - message end type (when I3C is acting as controller)
    #[inline(always)]
    pub fn mend(&mut self) -> MEND_W<I3C_CR_ALTERNATErs> {
        MEND_W::new(self, 31)
    }
}
/**I3C message control register alternate

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_cr_alternate::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_CR_ALTERNATE)*/
pub struct I3C_CR_ALTERNATErs;
impl crate::RegisterSpec for I3C_CR_ALTERNATErs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`i3c_cr_alternate::W`](W) writer structure
impl crate::Writable for I3C_CR_ALTERNATErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I3C_CR_ALTERNATE to value 0
impl crate::Resettable for I3C_CR_ALTERNATErs {
    const RESET_VALUE: u32 = 0;
}
