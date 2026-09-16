///Register `APB2ENR` reader
pub type R = crate::R<APB2ENRrs>;
///Register `APB2ENR` writer
pub type W = crate::W<APB2ENRrs>;
///Field `MRBLEEN` reader - MR_BLE enable
pub type MRBLEEN_R = crate::BitReader;
///Field `MRBLEEN` writer - MR_BLE enable
pub type MRBLEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKBLEDIV` reader - MR_BLE clock frequency selection when RCC_APB2ENR.MRBLEEN=1
pub type CLKBLEDIV_R = crate::BitReader;
///Field `CLKBLEDIV` writer - MR_BLE clock frequency selection when RCC_APB2ENR.MRBLEEN=1
pub type CLKBLEDIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MR_BLE enable
    #[inline(always)]
    pub fn mrbleen(&self) -> MRBLEEN_R {
        MRBLEEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - MR_BLE clock frequency selection when RCC_APB2ENR.MRBLEEN=1
    #[inline(always)]
    pub fn clkblediv(&self) -> CLKBLEDIV_R {
        CLKBLEDIV_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2ENR")
            .field("mrbleen", &self.mrbleen())
            .field("clkblediv", &self.clkblediv())
            .finish()
    }
}
impl W {
    ///Bit 0 - MR_BLE enable
    #[inline(always)]
    pub fn mrbleen(&mut self) -> MRBLEEN_W<'_, APB2ENRrs> {
        MRBLEEN_W::new(self, 0)
    }
    ///Bit 2 - MR_BLE clock frequency selection when RCC_APB2ENR.MRBLEEN=1
    #[inline(always)]
    pub fn clkblediv(&mut self) -> CLKBLEDIV_W<'_, APB2ENRrs> {
        CLKBLEDIV_W::new(self, 2)
    }
}
/**APB2ENR register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RCC:APB2ENR)*/
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2enr::R`](R) reader structure
impl crate::Readable for APB2ENRrs {}
///`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2ENR to value 0
impl crate::Resettable for APB2ENRrs {}
