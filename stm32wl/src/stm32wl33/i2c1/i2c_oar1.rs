///Register `I2C_OAR1` reader
pub type R = crate::R<I2C_OAR1rs>;
///Register `I2C_OAR1` writer
pub type W = crate::W<I2C_OAR1rs>;
///Field `OA1` reader - Interface address
pub type OA1_R = crate::FieldReader<u16>;
///Field `OA1` writer - Interface address
pub type OA1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `OA1MODE` reader - Own Address 1 10-bit mode - 0: Own address 1 is a 7-bit address. - 1: Own address 1 is a 10-bit address.
pub type OA1MODE_R = crate::BitReader;
///Field `OA1MODE` writer - Own Address 1 10-bit mode - 0: Own address 1 is a 7-bit address. - 1: Own address 1 is a 10-bit address.
pub type OA1MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OA1EN` reader - Own Address 1 enable - 0: Own address 1 disabled. The received slave address OA1 is NACKed. - 1: Own address 1 enabled. The received slave address OA1 is ACKed.
pub type OA1EN_R = crate::BitReader;
///Field `OA1EN` writer - Own Address 1 enable - 0: Own address 1 disabled. The received slave address OA1 is NACKed. - 1: Own address 1 enabled. The received slave address OA1 is ACKed.
pub type OA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    pub fn oa1(&self) -> OA1_R {
        OA1_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - Own Address 1 10-bit mode - 0: Own address 1 is a 7-bit address. - 1: Own address 1 is a 10-bit address.
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Own Address 1 enable - 0: Own address 1 disabled. The received slave address OA1 is NACKed. - 1: Own address 1 enabled. The received slave address OA1 is ACKed.
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_OAR1")
            .field("oa1", &self.oa1())
            .field("oa1mode", &self.oa1mode())
            .field("oa1en", &self.oa1en())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    pub fn oa1(&mut self) -> OA1_W<'_, I2C_OAR1rs> {
        OA1_W::new(self, 0)
    }
    ///Bit 10 - Own Address 1 10-bit mode - 0: Own address 1 is a 7-bit address. - 1: Own address 1 is a 10-bit address.
    #[inline(always)]
    pub fn oa1mode(&mut self) -> OA1MODE_W<'_, I2C_OAR1rs> {
        OA1MODE_W::new(self, 10)
    }
    ///Bit 15 - Own Address 1 enable - 0: Own address 1 disabled. The received slave address OA1 is NACKed. - 1: Own address 1 enabled. The received slave address OA1 is ACKed.
    #[inline(always)]
    pub fn oa1en(&mut self) -> OA1EN_W<'_, I2C_OAR1rs> {
        OA1EN_W::new(self, 15)
    }
}
/**I2C_OAR1 register

You can [`read`](crate::Reg::read) this register and get [`i2c_oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_OAR1)*/
pub struct I2C_OAR1rs;
impl crate::RegisterSpec for I2C_OAR1rs {
    type Ux = u32;
}
///`read()` method returns [`i2c_oar1::R`](R) reader structure
impl crate::Readable for I2C_OAR1rs {}
///`write(|w| ..)` method takes [`i2c_oar1::W`](W) writer structure
impl crate::Writable for I2C_OAR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C_OAR1 to value 0
impl crate::Resettable for I2C_OAR1rs {}
