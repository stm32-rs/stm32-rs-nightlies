///Register `SVMCR` reader
pub type R = crate::R<SVMCRrs>;
///Register `SVMCR` writer
pub type W = crate::W<SVMCRrs>;
///Field `PVDE` reader - Programmable voltage detector enable
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - Programmable voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDLS` reader - Programmable voltage detector level selection These bits select the voltage threshold detected by the programmable voltage detector:
pub type PVDLS_R = crate::FieldReader;
///Field `PVDLS` writer - Programmable voltage detector level selection These bits select the voltage threshold detected by the programmable voltage detector:
pub type PVDLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 4 - Programmable voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - Programmable voltage detector level selection These bits select the voltage threshold detected by the programmable voltage detector:
    #[inline(always)]
    pub fn pvdls(&self) -> PVDLS_R {
        PVDLS_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SVMCR")
            .field("pvde", &self.pvde())
            .field("pvdls", &self.pvdls())
            .finish()
    }
}
impl W {
    ///Bit 4 - Programmable voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, SVMCRrs> {
        PVDE_W::new(self, 4)
    }
    ///Bits 5:7 - Programmable voltage detector level selection These bits select the voltage threshold detected by the programmable voltage detector:
    #[inline(always)]
    pub fn pvdls(&mut self) -> PVDLS_W<'_, SVMCRrs> {
        PVDLS_W::new(self, 5)
    }
}
/**PWR supply voltage monitoring control register

You can [`read`](crate::Reg::read) this register and get [`svmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#PWR:SVMCR)*/
pub struct SVMCRrs;
impl crate::RegisterSpec for SVMCRrs {
    type Ux = u32;
}
///`read()` method returns [`svmcr::R`](R) reader structure
impl crate::Readable for SVMCRrs {}
///`write(|w| ..)` method takes [`svmcr::W`](W) writer structure
impl crate::Writable for SVMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SVMCR to value 0
impl crate::Resettable for SVMCRrs {}
