///Register `APB1HENR` reader
pub type R = crate::R<APB1HENRrs>;
///Register `APB1HENR` writer
pub type W = crate::W<APB1HENRrs>;
///Field `MDIOSEN` reader - MDIOS enable
pub type MDIOSEN_R = crate::BitReader;
///Field `MDIOSEN` writer - MDIOS enable
pub type MDIOSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANEN` reader - FDCAN enable
pub type FDCANEN_R = crate::BitReader;
///Field `FDCANEN` writer - FDCAN enable
pub type FDCANEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1EN` reader - UCPD1 enable
pub type UCPD1EN_R = crate::BitReader;
///Field `UCPD1EN` writer - UCPD1 enable
pub type UCPD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - MDIOS enable
    #[inline(always)]
    pub fn mdiosen(&self) -> MDIOSEN_R {
        MDIOSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FDCAN enable
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 18 - UCPD1 enable
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HENR")
            .field("mdiosen", &self.mdiosen())
            .field("fdcanen", &self.fdcanen())
            .field("ucpd1en", &self.ucpd1en())
            .finish()
    }
}
impl W {
    ///Bit 5 - MDIOS enable
    #[inline(always)]
    pub fn mdiosen(&mut self) -> MDIOSEN_W<'_, APB1HENRrs> {
        MDIOSEN_W::new(self, 5)
    }
    ///Bit 8 - FDCAN enable
    #[inline(always)]
    pub fn fdcanen(&mut self) -> FDCANEN_W<'_, APB1HENRrs> {
        FDCANEN_W::new(self, 8)
    }
    ///Bit 18 - UCPD1 enable
    #[inline(always)]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W<'_, APB1HENRrs> {
        UCPD1EN_W::new(self, 18)
    }
}
/**RCC APB1H enable register

You can [`read`](crate::Reg::read) this register and get [`apb1henr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1henr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB1HENR)*/
pub struct APB1HENRrs;
impl crate::RegisterSpec for APB1HENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1henr::R`](R) reader structure
impl crate::Readable for APB1HENRrs {}
///`write(|w| ..)` method takes [`apb1henr::W`](W) writer structure
impl crate::Writable for APB1HENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HENR to value 0
impl crate::Resettable for APB1HENRrs {}
