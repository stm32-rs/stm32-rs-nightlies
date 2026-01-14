///Register `CR_alternate` writer
pub type W = crate::W<CR_ALTERNATErs>;
///Field `DCNT` writer - Count of related data to the CCC command to transfer as CCC defining bytes, or CCC sub-command bytes, or CCC data bytes, in bytes
pub type DCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CCC` writer - 8-bit CCC code (when I3C acts as controller)
pub type CCC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MTYPE` writer - Message type (when I3C acts as controller)
pub type MTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MEND` writer - Message end type / last message of a frame (when I3C acts as controller)
pub type MEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CR_ALTERNATErs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - Count of related data to the CCC command to transfer as CCC defining bytes, or CCC sub-command bytes, or CCC data bytes, in bytes
    #[inline(always)]
    pub fn dcnt(&mut self) -> DCNT_W<'_, CR_ALTERNATErs> {
        DCNT_W::new(self, 0)
    }
    ///Bits 16:23 - 8-bit CCC code (when I3C acts as controller)
    #[inline(always)]
    pub fn ccc(&mut self) -> CCC_W<'_, CR_ALTERNATErs> {
        CCC_W::new(self, 16)
    }
    ///Bits 27:30 - Message type (when I3C acts as controller)
    #[inline(always)]
    pub fn mtype(&mut self) -> MTYPE_W<'_, CR_ALTERNATErs> {
        MTYPE_W::new(self, 27)
    }
    ///Bit 31 - Message end type / last message of a frame (when I3C acts as controller)
    #[inline(always)]
    pub fn mend(&mut self) -> MEND_W<'_, CR_ALTERNATErs> {
        MEND_W::new(self, 31)
    }
}
/**I3C message control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_alternate::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#I3C1:CR_alternate)*/
pub struct CR_ALTERNATErs;
impl crate::RegisterSpec for CR_ALTERNATErs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cr_alternate::W`](W) writer structure
impl crate::Writable for CR_ALTERNATErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR_alternate to value 0
impl crate::Resettable for CR_ALTERNATErs {}
