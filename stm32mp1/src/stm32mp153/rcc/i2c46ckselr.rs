///Register `I2C46CKSELR` reader
pub type R = crate::R<I2C46CKSELRrs>;
///Register `I2C46CKSELR` writer
pub type W = crate::W<I2C46CKSELRrs>;
///Field `I2C46SRC` reader - I2C46SRC
pub type I2C46SRC_R = crate::FieldReader;
///Field `I2C46SRC` writer - I2C46SRC
pub type I2C46SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - I2C46SRC
    #[inline(always)]
    pub fn i2c46src(&self) -> I2C46SRC_R {
        I2C46SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C46CKSELR")
            .field("i2c46src", &self.i2c46src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - I2C46SRC
    #[inline(always)]
    pub fn i2c46src(&mut self) -> I2C46SRC_W<'_, I2C46CKSELRrs> {
        I2C46SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`i2c46ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c46ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:I2C46CKSELR)*/
pub struct I2C46CKSELRrs;
impl crate::RegisterSpec for I2C46CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`i2c46ckselr::R`](R) reader structure
impl crate::Readable for I2C46CKSELRrs {}
///`write(|w| ..)` method takes [`i2c46ckselr::W`](W) writer structure
impl crate::Writable for I2C46CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C46CKSELR to value 0
impl crate::Resettable for I2C46CKSELRrs {}
