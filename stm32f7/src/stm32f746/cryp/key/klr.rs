///Register `KLR` writer
pub type W = crate::W<KLRrs>;
///Field `b2` writer - b224
pub type B2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KLRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - b224
    #[inline(always)]
    pub fn b2(&mut self) -> B2_W<'_, KLRrs> {
        B2_W::new(self, 0)
    }
}
/**key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`klr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct KLRrs;
impl crate::RegisterSpec for KLRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`klr::W`](W) writer structure
impl crate::Writable for KLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KLR to value 0
impl crate::Resettable for KLRrs {}
