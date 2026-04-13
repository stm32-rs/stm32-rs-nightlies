///Register `APB1DIVR` reader
pub type R = crate::R<APB1DIVRrs>;
///Register `APB1DIVR` writer
pub type W = crate::W<APB1DIVRrs>;
///Field `APB1DIV` reader - APB1DIV
pub type APB1DIV_R = crate::FieldReader;
///Field `APB1DIV` writer - APB1DIV
pub type APB1DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `APB1DIVRDY` reader - APB1DIVRDY
pub type APB1DIVRDY_R = crate::BitReader;
impl R {
    ///Bits 0:2 - APB1DIV
    #[inline(always)]
    pub fn apb1div(&self) -> APB1DIV_R {
        APB1DIV_R::new((self.bits & 7) as u8)
    }
    ///Bit 31 - APB1DIVRDY
    #[inline(always)]
    pub fn apb1divrdy(&self) -> APB1DIVRDY_R {
        APB1DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1DIVR")
            .field("apb1div", &self.apb1div())
            .field("apb1divrdy", &self.apb1divrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - APB1DIV
    #[inline(always)]
    pub fn apb1div(&mut self) -> APB1DIV_W<'_, APB1DIVRrs> {
        APB1DIV_W::new(self, 0)
    }
}
/**This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information.

You can [`read`](crate::Reg::read) this register and get [`apb1divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:APB1DIVR)*/
pub struct APB1DIVRrs;
impl crate::RegisterSpec for APB1DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1divr::R`](R) reader structure
impl crate::Readable for APB1DIVRrs {}
///`write(|w| ..)` method takes [`apb1divr::W`](W) writer structure
impl crate::Writable for APB1DIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1DIVR to value 0x8000_0000
impl crate::Resettable for APB1DIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
