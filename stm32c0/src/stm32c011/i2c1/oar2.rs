///Register `OAR2` reader
pub type R = crate::R<OAR2rs>;
///Register `OAR2` writer
pub type W = crate::W<OAR2rs>;
///Field `OA2` reader - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0.
pub type OA2_R = crate::FieldReader;
///Field `OA2` writer - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0.
pub type OA2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `OA2MSK` reader - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches.
pub type OA2MSK_R = crate::FieldReader;
///Field `OA2MSK` writer - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches.
pub type OA2MSK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OA2EN` reader - Own Address 2 enable
pub type OA2EN_R = crate::BitReader;
///Field `OA2EN` writer - Own Address 2 enable
pub type OA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 1:7 - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0.
    #[inline(always)]
    pub fn oa2(&self) -> OA2_R {
        OA2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bits 8:10 - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches.
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 15 - Own Address 2 enable
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OAR2")
            .field("oa2", &self.oa2())
            .field("oa2msk", &self.oa2msk())
            .field("oa2en", &self.oa2en())
            .finish()
    }
}
impl W {
    ///Bits 1:7 - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0.
    #[inline(always)]
    pub fn oa2(&mut self) -> OA2_W<OAR2rs> {
        OA2_W::new(self, 1)
    }
    ///Bits 8:10 - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches.
    #[inline(always)]
    pub fn oa2msk(&mut self) -> OA2MSK_W<OAR2rs> {
        OA2MSK_W::new(self, 8)
    }
    ///Bit 15 - Own Address 2 enable
    #[inline(always)]
    pub fn oa2en(&mut self) -> OA2EN_W<OAR2rs> {
        OA2EN_W::new(self, 15)
    }
}
/**I2C own address 2 register

You can [`read`](crate::Reg::read) this register and get [`oar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#I2C1:OAR2)*/
pub struct OAR2rs;
impl crate::RegisterSpec for OAR2rs {
    type Ux = u32;
}
///`read()` method returns [`oar2::R`](R) reader structure
impl crate::Readable for OAR2rs {}
///`write(|w| ..)` method takes [`oar2::W`](W) writer structure
impl crate::Writable for OAR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OAR2 to value 0
impl crate::Resettable for OAR2rs {
    const RESET_VALUE: u32 = 0;
}
