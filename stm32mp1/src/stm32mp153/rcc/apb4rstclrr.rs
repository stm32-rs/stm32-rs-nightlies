///Register `APB4RSTCLRR` reader
pub type R = crate::R<APB4RSTCLRRrs>;
///Register `APB4RSTCLRR` writer
pub type W = crate::W<APB4RSTCLRRrs>;
///Field `LTDCRST` reader - LTDCRST
pub type LTDCRST_R = crate::BitReader;
///Field `LTDCRST` writer - LTDCRST
pub type LTDCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSIRST` reader - DSIRST
pub type DSIRST_R = crate::BitReader;
///Field `DSIRST` writer - DSIRST
pub type DSIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRPERFMRST` reader - DDRPERFMRST
pub type DDRPERFMRST_R = crate::BitReader;
///Field `DDRPERFMRST` writer - DDRPERFMRST
pub type DDRPERFMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBPHYRST` reader - USBPHYRST
pub type USBPHYRST_R = crate::BitReader;
///Field `USBPHYRST` writer - USBPHYRST
pub type USBPHYRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LTDCRST
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DSIRST
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - DDRPERFMRST
    #[inline(always)]
    pub fn ddrperfmrst(&self) -> DDRPERFMRST_R {
        DDRPERFMRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - USBPHYRST
    #[inline(always)]
    pub fn usbphyrst(&self) -> USBPHYRST_R {
        USBPHYRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4RSTCLRR")
            .field("ltdcrst", &self.ltdcrst())
            .field("dsirst", &self.dsirst())
            .field("ddrperfmrst", &self.ddrperfmrst())
            .field("usbphyrst", &self.usbphyrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - LTDCRST
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<'_, APB4RSTCLRRrs> {
        LTDCRST_W::new(self, 0)
    }
    ///Bit 4 - DSIRST
    #[inline(always)]
    pub fn dsirst(&mut self) -> DSIRST_W<'_, APB4RSTCLRRrs> {
        DSIRST_W::new(self, 4)
    }
    ///Bit 8 - DDRPERFMRST
    #[inline(always)]
    pub fn ddrperfmrst(&mut self) -> DDRPERFMRST_W<'_, APB4RSTCLRRrs> {
        DDRPERFMRST_W::new(self, 8)
    }
    ///Bit 16 - USBPHYRST
    #[inline(always)]
    pub fn usbphyrst(&mut self) -> USBPHYRST_W<'_, APB4RSTCLRRrs> {
        USBPHYRST_W::new(self, 16)
    }
}
/**This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`apb4rstclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4rstclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB4RSTCLRR)*/
pub struct APB4RSTCLRRrs;
impl crate::RegisterSpec for APB4RSTCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4rstclrr::R`](R) reader structure
impl crate::Readable for APB4RSTCLRRrs {}
///`write(|w| ..)` method takes [`apb4rstclrr::W`](W) writer structure
impl crate::Writable for APB4RSTCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4RSTCLRR to value 0
impl crate::Resettable for APB4RSTCLRRrs {}
