///Register `PR2` reader
pub type R = crate::R<PR2rs>;
///Register `PR2` writer
pub type W = crate::W<PR2rs>;
///Field `PR34` reader - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
pub type PR34_R = crate::BitReader;
///Field `PR34` writer - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
pub type PR34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PR46` reader - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
pub type PR46_R = crate::BitReader;
///Field `PR46` writer - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
pub type PR46_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PR49` reader - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
pub type PR49_R = crate::BitReader;
///Field `PR49` writer - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
pub type PR49_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PR51` reader - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
pub type PR51_R = crate::BitReader;
///Field `PR51` writer - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
pub type PR51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PR54` reader - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
pub type PR54_R = crate::BitReader;
///Field `PR54` writer - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
pub type PR54_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
    #[inline(always)]
    pub fn pr34(&self) -> PR34_R {
        PR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 14 - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
    #[inline(always)]
    pub fn pr46(&self) -> PR46_R {
        PR46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
    #[inline(always)]
    pub fn pr49(&self) -> PR49_R {
        PR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
    #[inline(always)]
    pub fn pr51(&self) -> PR51_R {
        PR51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
    #[inline(always)]
    pub fn pr54(&self) -> PR54_R {
        PR54_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR2")
            .field("pr34", &self.pr34())
            .field("pr46", &self.pr46())
            .field("pr49", &self.pr49())
            .field("pr51", &self.pr51())
            .field("pr54", &self.pr54())
            .finish()
    }
}
impl W {
    ///Bit 2 - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
    #[inline(always)]
    pub fn pr34(&mut self) -> PR34_W<'_, PR2rs> {
        PR34_W::new(self, 2)
    }
    ///Bit 14 - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
    #[inline(always)]
    pub fn pr46(&mut self) -> PR46_W<'_, PR2rs> {
        PR46_W::new(self, 14)
    }
    ///Bit 17 - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
    #[inline(always)]
    pub fn pr49(&mut self) -> PR49_W<'_, PR2rs> {
        PR49_W::new(self, 17)
    }
    ///Bit 19 - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
    #[inline(always)]
    pub fn pr51(&mut self) -> PR51_W<'_, PR2rs> {
        PR51_W::new(self, 19)
    }
    ///Bit 22 - Configurable event inputs x+32 Pending bit This bit is set when the selected edge event arrives on the external interrupt line. This bit is cleared by writing a 1 into the bit or by changing the sensitivity of the edge detector.
    #[inline(always)]
    pub fn pr54(&mut self) -> PR54_W<'_, PR2rs> {
        PR54_W::new(self, 22)
    }
}
/**EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`pr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#EXTI:PR2)*/
pub struct PR2rs;
impl crate::RegisterSpec for PR2rs {
    type Ux = u32;
}
///`read()` method returns [`pr2::R`](R) reader structure
impl crate::Readable for PR2rs {}
///`write(|w| ..)` method takes [`pr2::W`](W) writer structure
impl crate::Writable for PR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PR2 to value 0
impl crate::Resettable for PR2rs {}
