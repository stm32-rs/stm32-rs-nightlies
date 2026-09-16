///Register `I2C12CKSELR` reader
pub type R = crate::R<I2C12CKSELRrs>;
///Register `I2C12CKSELR` writer
pub type W = crate::W<I2C12CKSELRrs>;
///Field `I2C12SRC` reader - I2C12SRC
pub type I2C12SRC_R = crate::FieldReader;
///Field `I2C12SRC` writer - I2C12SRC
pub type I2C12SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - I2C12SRC
    #[inline(always)]
    pub fn i2c12src(&self) -> I2C12SRC_R {
        I2C12SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C12CKSELR")
            .field("i2c12src", &self.i2c12src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - I2C12SRC
    #[inline(always)]
    pub fn i2c12src(&mut self) -> I2C12SRC_W<'_, I2C12CKSELRrs> {
        I2C12SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`i2c12ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c12ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:I2C12CKSELR)*/
pub struct I2C12CKSELRrs;
impl crate::RegisterSpec for I2C12CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`i2c12ckselr::R`](R) reader structure
impl crate::Readable for I2C12CKSELRrs {}
///`write(|w| ..)` method takes [`i2c12ckselr::W`](W) writer structure
impl crate::Writable for I2C12CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C12CKSELR to value 0
impl crate::Resettable for I2C12CKSELRrs {}
