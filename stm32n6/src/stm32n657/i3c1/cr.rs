///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DCNT` writer - Count of data to transfer during a read or write message, in bytes (whatever I3C acts as controller/target)
pub type DCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RNW` writer - Read / non-write message (when I3C acts as controller)
pub type RNW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD` writer - 7-bit I3C dynamic / Iless thansup>2less than/sup>C static target address (when I3C acts as controller)
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `MTYPE` writer - Message type (whatever I3C acts as controller/target)
pub type MTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MEND` writer - Message end type / last message of a frame (when the I3C acts as controller)
pub type MEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - Count of data to transfer during a read or write message, in bytes (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn dcnt(&mut self) -> DCNT_W<'_, CRrs> {
        DCNT_W::new(self, 0)
    }
    ///Bit 16 - Read / non-write message (when I3C acts as controller)
    #[inline(always)]
    pub fn rnw(&mut self) -> RNW_W<'_, CRrs> {
        RNW_W::new(self, 16)
    }
    ///Bits 17:23 - 7-bit I3C dynamic / Iless thansup>2less than/sup>C static target address (when I3C acts as controller)
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, CRrs> {
        ADD_W::new(self, 17)
    }
    ///Bits 27:30 - Message type (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn mtype(&mut self) -> MTYPE_W<'_, CRrs> {
        MTYPE_W::new(self, 27)
    }
    ///Bit 31 - Message end type / last message of a frame (when the I3C acts as controller)
    #[inline(always)]
    pub fn mend(&mut self) -> MEND_W<'_, CRrs> {
        MEND_W::new(self, 31)
    }
}
/**I3C message control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#I3C1:CR)*/
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
