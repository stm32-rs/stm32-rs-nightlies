///Register `RDCR` reader
pub type R = crate::R<RDCRrs>;
///Register `RDCR` writer
pub type W = crate::W<RDCRrs>;
///Field `MRD` reader - BOOTROM sleep enable
pub type MRD_R = crate::FieldReader;
///Field `MRD` writer - BOOTROM sleep enable
pub type MRD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EADLY` reader - BOOTROM sleep enable
pub type EADLY_R = crate::FieldReader;
///Field `EADLY` writer - BOOTROM sleep enable
pub type EADLY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 16:20 - BOOTROM sleep enable
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:27 - BOOTROM sleep enable
    #[inline(always)]
    pub fn eadly(&self) -> EADLY_R {
        EADLY_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDCR")
            .field("mrd", &self.mrd())
            .field("eadly", &self.eadly())
            .finish()
    }
}
impl W {
    ///Bits 16:20 - BOOTROM sleep enable
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W<'_, RDCRrs> {
        MRD_W::new(self, 16)
    }
    ///Bits 24:27 - BOOTROM sleep enable
    #[inline(always)]
    pub fn eadly(&mut self) -> EADLY_W<'_, RDCRrs> {
        EADLY_W::new(self, 24)
    }
}
/**RCC APB5 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`rdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:RDCR)*/
pub struct RDCRrs;
impl crate::RegisterSpec for RDCRrs {
    type Ux = u32;
}
///`read()` method returns [`rdcr::R`](R) reader structure
impl crate::Readable for RDCRrs {}
///`write(|w| ..)` method takes [`rdcr::W`](W) writer structure
impl crate::Writable for RDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RDCR to value 0x0006_0000
impl crate::Resettable for RDCRrs {
    const RESET_VALUE: u32 = 0x0006_0000;
}
