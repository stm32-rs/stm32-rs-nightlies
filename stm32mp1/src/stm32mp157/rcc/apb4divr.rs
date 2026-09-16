///Register `APB4DIVR` reader
pub type R = crate::R<APB4DIVRrs>;
///Register `APB4DIVR` writer
pub type W = crate::W<APB4DIVRrs>;
///Field `APB4DIV` reader - APB4DIV
pub type APB4DIV_R = crate::FieldReader;
///Field `APB4DIV` writer - APB4DIV
pub type APB4DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `APB4DIVRDY` reader - APB4DIVRDY
pub type APB4DIVRDY_R = crate::BitReader;
impl R {
    ///Bits 0:2 - APB4DIV
    #[inline(always)]
    pub fn apb4div(&self) -> APB4DIV_R {
        APB4DIV_R::new((self.bits & 7) as u8)
    }
    ///Bit 31 - APB4DIVRDY
    #[inline(always)]
    pub fn apb4divrdy(&self) -> APB4DIVRDY_R {
        APB4DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4DIVR")
            .field("apb4div", &self.apb4div())
            .field("apb4divrdy", &self.apb4divrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - APB4DIV
    #[inline(always)]
    pub fn apb4div(&mut self) -> APB4DIV_W<'_, APB4DIVRrs> {
        APB4DIV_W::new(self, 0)
    }
}
/**This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`apb4divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:APB4DIVR)*/
pub struct APB4DIVRrs;
impl crate::RegisterSpec for APB4DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4divr::R`](R) reader structure
impl crate::Readable for APB4DIVRrs {}
///`write(|w| ..)` method takes [`apb4divr::W`](W) writer structure
impl crate::Writable for APB4DIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4DIVR to value 0x8000_0000
impl crate::Resettable for APB4DIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
