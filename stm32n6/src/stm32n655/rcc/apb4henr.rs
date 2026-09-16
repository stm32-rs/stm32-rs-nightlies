///Register `APB4HENR` reader
pub type R = crate::R<APB4HENRrs>;
///Register `APB4HENR` writer
pub type W = crate::W<APB4HENRrs>;
///Field `SYSCFGEN` reader - SYSCFG enable
pub type SYSCFGEN_R = crate::BitReader;
///Field `SYSCFGEN` writer - SYSCFG enable
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSECEN` reader - BSEC enable
pub type BSECEN_R = crate::BitReader;
///Field `BSECEN` writer - BSEC enable
pub type BSECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSEN` reader - DTS enable
pub type DTSEN_R = crate::BitReader;
///Field `DTSEN` writer - DTS enable
pub type DTSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPERFMEN` reader - BUSPERFM enable
pub type BUSPERFMEN_R = crate::BitReader;
///Field `BUSPERFMEN` writer - BUSPERFM enable
pub type BUSPERFMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SYSCFG enable
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BSEC enable
    #[inline(always)]
    pub fn bsecen(&self) -> BSECEN_R {
        BSECEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DTS enable
    #[inline(always)]
    pub fn dtsen(&self) -> DTSEN_R {
        DTSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - BUSPERFM enable
    #[inline(always)]
    pub fn busperfmen(&self) -> BUSPERFMEN_R {
        BUSPERFMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4HENR")
            .field("syscfgen", &self.syscfgen())
            .field("bsecen", &self.bsecen())
            .field("dtsen", &self.dtsen())
            .field("busperfmen", &self.busperfmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG enable
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, APB4HENRrs> {
        SYSCFGEN_W::new(self, 0)
    }
    ///Bit 1 - BSEC enable
    #[inline(always)]
    pub fn bsecen(&mut self) -> BSECEN_W<'_, APB4HENRrs> {
        BSECEN_W::new(self, 1)
    }
    ///Bit 2 - DTS enable
    #[inline(always)]
    pub fn dtsen(&mut self) -> DTSEN_W<'_, APB4HENRrs> {
        DTSEN_W::new(self, 2)
    }
    ///Bit 4 - BUSPERFM enable
    #[inline(always)]
    pub fn busperfmen(&mut self) -> BUSPERFMEN_W<'_, APB4HENRrs> {
        BUSPERFMEN_W::new(self, 4)
    }
}
/**RCC APB4H enable register

You can [`read`](crate::Reg::read) this register and get [`apb4henr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4henr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB4HENR)*/
pub struct APB4HENRrs;
impl crate::RegisterSpec for APB4HENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4henr::R`](R) reader structure
impl crate::Readable for APB4HENRrs {}
///`write(|w| ..)` method takes [`apb4henr::W`](W) writer structure
impl crate::Writable for APB4HENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4HENR to value 0x02
impl crate::Resettable for APB4HENRrs {
    const RESET_VALUE: u32 = 0x02;
}
