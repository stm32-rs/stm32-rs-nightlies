///Register `DHR12LD` reader
pub type R = crate::R<DHR12LDrs>;
///Register `DHR12LD` writer
pub type W = crate::W<DHR12LDrs>;
///Field `DACC1DHR` reader - DACC1DHR
pub type DACC1DHR_R = crate::FieldReader<u16>;
///Field `DACC1DHR` writer - DACC1DHR
pub type DACC1DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `DACC2DHR` reader - DACC2DHR
pub type DACC2DHR_R = crate::FieldReader<u16>;
///Field `DACC2DHR` writer - DACC2DHR
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 4:15 - DACC1DHR
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    ///Bits 20:31 - DACC2DHR
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR12LD")
            .field("dacc1dhr", &self.dacc1dhr())
            .field("dacc2dhr", &self.dacc2dhr())
            .finish()
    }
}
impl W {
    ///Bits 4:15 - DACC1DHR
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<'_, DHR12LDrs> {
        DACC1DHR_W::new(self, 4)
    }
    ///Bits 20:31 - DACC2DHR
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<'_, DHR12LDrs> {
        DACC2DHR_W::new(self, 20)
    }
}
/**Dual DAC 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12ld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12ld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DAC1:DHR12LD)*/
pub struct DHR12LDrs;
impl crate::RegisterSpec for DHR12LDrs {
    type Ux = u32;
}
///`read()` method returns [`dhr12ld::R`](R) reader structure
impl crate::Readable for DHR12LDrs {}
///`write(|w| ..)` method takes [`dhr12ld::W`](W) writer structure
impl crate::Writable for DHR12LDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR12LD to value 0
impl crate::Resettable for DHR12LDrs {}
