///Register `APB2DIVR` reader
pub type R = crate::R<APB2DIVRrs>;
///Register `APB2DIVR` writer
pub type W = crate::W<APB2DIVRrs>;
///Field `APB2DIV` reader - APB2DIV
pub type APB2DIV_R = crate::FieldReader;
///Field `APB2DIV` writer - APB2DIV
pub type APB2DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `APB2DIVRDY` reader - APB2DIVRDY
pub type APB2DIVRDY_R = crate::BitReader;
impl R {
    ///Bits 0:2 - APB2DIV
    #[inline(always)]
    pub fn apb2div(&self) -> APB2DIV_R {
        APB2DIV_R::new((self.bits & 7) as u8)
    }
    ///Bit 31 - APB2DIVRDY
    #[inline(always)]
    pub fn apb2divrdy(&self) -> APB2DIVRDY_R {
        APB2DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2DIVR")
            .field("apb2div", &self.apb2div())
            .field("apb2divrdy", &self.apb2divrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - APB2DIV
    #[inline(always)]
    pub fn apb2div(&mut self) -> APB2DIV_W<'_, APB2DIVRrs> {
        APB2DIV_W::new(self, 0)
    }
}
/**This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.

You can [`read`](crate::Reg::read) this register and get [`apb2divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB2DIVR)*/
pub struct APB2DIVRrs;
impl crate::RegisterSpec for APB2DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2divr::R`](R) reader structure
impl crate::Readable for APB2DIVRrs {}
///`write(|w| ..)` method takes [`apb2divr::W`](W) writer structure
impl crate::Writable for APB2DIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2DIVR to value 0x8000_0000
impl crate::Resettable for APB2DIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
