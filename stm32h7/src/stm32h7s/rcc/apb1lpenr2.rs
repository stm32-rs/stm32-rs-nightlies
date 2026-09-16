///Register `APB1LPENR2` reader
pub type R = crate::R<APB1LPENR2rs>;
///Register `APB1LPENR2` writer
pub type W = crate::W<APB1LPENR2rs>;
///Field `CRSLPEN` reader - clock recovery system peripheral clock enable in low-power mode
pub type CRSLPEN_R = crate::BitReader;
///Field `CRSLPEN` writer - clock recovery system peripheral clock enable in low-power mode
pub type CRSLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDIOSLPEN` reader - MDIOS peripheral clock enable in low-power mode
pub type MDIOSLPEN_R = crate::BitReader;
///Field `MDIOSLPEN` writer - MDIOS peripheral clock enable in low-power mode
pub type MDIOSLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANLPEN` reader - FDCAN peripheral clock enable in low-power mode
pub type FDCANLPEN_R = crate::BitReader;
///Field `FDCANLPEN` writer - FDCAN peripheral clock enable in low-power mode
pub type FDCANLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1LPEN` reader - UCPD peripheral clock enable in low-power mode
pub type UCPD1LPEN_R = crate::BitReader;
///Field `UCPD1LPEN` writer - UCPD peripheral clock enable in low-power mode
pub type UCPD1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - clock recovery system peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn crslpen(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - MDIOS peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn mdioslpen(&self) -> MDIOSLPEN_R {
        MDIOSLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FDCAN peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 27 - UCPD peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn ucpd1lpen(&self) -> UCPD1LPEN_R {
        UCPD1LPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LPENR2")
            .field("crslpen", &self.crslpen())
            .field("mdioslpen", &self.mdioslpen())
            .field("fdcanlpen", &self.fdcanlpen())
            .field("ucpd1lpen", &self.ucpd1lpen())
            .finish()
    }
}
impl W {
    ///Bit 1 - clock recovery system peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn crslpen(&mut self) -> CRSLPEN_W<'_, APB1LPENR2rs> {
        CRSLPEN_W::new(self, 1)
    }
    ///Bit 5 - MDIOS peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn mdioslpen(&mut self) -> MDIOSLPEN_W<'_, APB1LPENR2rs> {
        MDIOSLPEN_W::new(self, 5)
    }
    ///Bit 8 - FDCAN peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<'_, APB1LPENR2rs> {
        FDCANLPEN_W::new(self, 8)
    }
    ///Bit 27 - UCPD peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn ucpd1lpen(&mut self) -> UCPD1LPEN_W<'_, APB1LPENR2rs> {
        UCPD1LPEN_W::new(self, 27)
    }
}
/**RCC APB1 low-power clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1lpenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lpenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:APB1LPENR2)*/
pub struct APB1LPENR2rs;
impl crate::RegisterSpec for APB1LPENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1lpenr2::R`](R) reader structure
impl crate::Readable for APB1LPENR2rs {}
///`write(|w| ..)` method takes [`apb1lpenr2::W`](W) writer structure
impl crate::Writable for APB1LPENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LPENR2 to value 0x0800_0122
impl crate::Resettable for APB1LPENR2rs {
    const RESET_VALUE: u32 = 0x0800_0122;
}
