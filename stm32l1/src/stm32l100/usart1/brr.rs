///Register `BRR` reader
pub type R = crate::R<BRRrs>;
///Register `BRR` writer
pub type W = crate::W<BRRrs>;
///Field `DIV_Fraction` reader - fraction of USARTDIV
pub type DIV_FRACTION_R = crate::FieldReader;
///Field `DIV_Fraction` writer - fraction of USARTDIV
pub type DIV_FRACTION_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `DIV_Mantissa` reader - mantissa of USARTDIV
pub type DIV_MANTISSA_R = crate::FieldReader<u16>;
///Field `DIV_Mantissa` writer - mantissa of USARTDIV
pub type DIV_MANTISSA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:3 - fraction of USARTDIV
    #[inline(always)]
    pub fn div_fraction(&self) -> DIV_FRACTION_R {
        DIV_FRACTION_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:15 - mantissa of USARTDIV
    #[inline(always)]
    pub fn div_mantissa(&self) -> DIV_MANTISSA_R {
        DIV_MANTISSA_R::new((self.bits >> 4) & 0x0fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRR")
            .field("div_mantissa", &self.div_mantissa())
            .field("div_fraction", &self.div_fraction())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - fraction of USARTDIV
    #[inline(always)]
    pub fn div_fraction(&mut self) -> DIV_FRACTION_W<'_, BRRrs> {
        DIV_FRACTION_W::new(self, 0)
    }
    ///Bits 4:15 - mantissa of USARTDIV
    #[inline(always)]
    pub fn div_mantissa(&mut self) -> DIV_MANTISSA_W<'_, BRRrs> {
        DIV_MANTISSA_W::new(self, 4)
    }
}
/**Baud rate register

You can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#USART1:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u16;
}
///`read()` method returns [`brr::R`](R) reader structure
impl crate::Readable for BRRrs {}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {}
