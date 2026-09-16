///Register `APB1ENR2` reader
pub type R = crate::R<APB1ENR2rs>;
///Register `APB1ENR2` writer
pub type W = crate::W<APB1ENR2rs>;
///Field `CRSEN` reader - clock recovery system peripheral clock enable
pub type CRSEN_R = crate::BitReader;
///Field `CRSEN` writer - clock recovery system peripheral clock enable
pub type CRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDIOSEN` reader - MDIOS peripheral clock enable
pub type MDIOSEN_R = crate::BitReader;
///Field `MDIOSEN` writer - MDIOS peripheral clock enable
pub type MDIOSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANEN` reader - FDCAN peripheral clock enable
pub type FDCANEN_R = crate::BitReader;
///Field `FDCANEN` writer - FDCAN peripheral clock enable
pub type FDCANEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1EN` reader - UCPD peripheral clock enable
pub type UCPD1EN_R = crate::BitReader;
///Field `UCPD1EN` writer - UCPD peripheral clock enable
pub type UCPD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - clock recovery system peripheral clock enable
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - MDIOS peripheral clock enable
    #[inline(always)]
    pub fn mdiosen(&self) -> MDIOSEN_R {
        MDIOSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FDCAN peripheral clock enable
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 27 - UCPD peripheral clock enable
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR2")
            .field("crsen", &self.crsen())
            .field("mdiosen", &self.mdiosen())
            .field("fdcanen", &self.fdcanen())
            .field("ucpd1en", &self.ucpd1en())
            .finish()
    }
}
impl W {
    ///Bit 1 - clock recovery system peripheral clock enable
    #[inline(always)]
    pub fn crsen(&mut self) -> CRSEN_W<'_, APB1ENR2rs> {
        CRSEN_W::new(self, 1)
    }
    ///Bit 5 - MDIOS peripheral clock enable
    #[inline(always)]
    pub fn mdiosen(&mut self) -> MDIOSEN_W<'_, APB1ENR2rs> {
        MDIOSEN_W::new(self, 5)
    }
    ///Bit 8 - FDCAN peripheral clock enable
    #[inline(always)]
    pub fn fdcanen(&mut self) -> FDCANEN_W<'_, APB1ENR2rs> {
        FDCANEN_W::new(self, 8)
    }
    ///Bit 27 - UCPD peripheral clock enable
    #[inline(always)]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W<'_, APB1ENR2rs> {
        UCPD1EN_W::new(self, 27)
    }
}
/**RCC APB1 clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB1ENR2)*/
pub struct APB1ENR2rs;
impl crate::RegisterSpec for APB1ENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1enr2::R`](R) reader structure
impl crate::Readable for APB1ENR2rs {}
///`write(|w| ..)` method takes [`apb1enr2::W`](W) writer structure
impl crate::Writable for APB1ENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1ENR2 to value 0
impl crate::Resettable for APB1ENR2rs {}
