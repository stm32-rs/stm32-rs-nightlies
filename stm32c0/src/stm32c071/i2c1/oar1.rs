///Register `OAR1` reader
pub type R = crate::R<OAR1rs>;
///Register `OAR1` writer
pub type W = crate::W<OAR1rs>;
/**Field `OA1` reader - Interface own slave address 7-bit addressing mode: OA1\[7:1\]
contains the 7-bit own slave address. Bits OA1\[9\], OA1\[8\]
and OA1\[0\]
are don't care. 10-bit addressing mode: OA1\[9:0\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN = 0.*/
pub type OA1_R = crate::FieldReader<u16>;
/**Field `OA1` writer - Interface own slave address 7-bit addressing mode: OA1\[7:1\]
contains the 7-bit own slave address. Bits OA1\[9\], OA1\[8\]
and OA1\[0\]
are don't care. 10-bit addressing mode: OA1\[9:0\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN = 0.*/
pub type OA1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `OA1MODE` reader - Own address 1 10-bit mode Note: This bit can be written only when OA1EN = 0.
pub type OA1MODE_R = crate::BitReader;
///Field `OA1MODE` writer - Own address 1 10-bit mode Note: This bit can be written only when OA1EN = 0.
pub type OA1MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OA1EN` reader - Own address 1 enable
pub type OA1EN_R = crate::BitReader;
///Field `OA1EN` writer - Own address 1 enable
pub type OA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\[7:1\]
    contains the 7-bit own slave address. Bits OA1\[9\], OA1\[8\]
    and OA1\[0\]
    are don't care. 10-bit addressing mode: OA1\[9:0\]
    contains the 10-bit own slave address. Note: These bits can be written only when OA1EN = 0.*/
    #[inline(always)]
    pub fn oa1(&self) -> OA1_R {
        OA1_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - Own address 1 10-bit mode Note: This bit can be written only when OA1EN = 0.
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Own address 1 enable
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OAR1")
            .field("oa1", &self.oa1())
            .field("oa1mode", &self.oa1mode())
            .field("oa1en", &self.oa1en())
            .finish()
    }
}
impl W {
    /**Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\[7:1\]
    contains the 7-bit own slave address. Bits OA1\[9\], OA1\[8\]
    and OA1\[0\]
    are don't care. 10-bit addressing mode: OA1\[9:0\]
    contains the 10-bit own slave address. Note: These bits can be written only when OA1EN = 0.*/
    #[inline(always)]
    pub fn oa1(&mut self) -> OA1_W<OAR1rs> {
        OA1_W::new(self, 0)
    }
    ///Bit 10 - Own address 1 10-bit mode Note: This bit can be written only when OA1EN = 0.
    #[inline(always)]
    pub fn oa1mode(&mut self) -> OA1MODE_W<OAR1rs> {
        OA1MODE_W::new(self, 10)
    }
    ///Bit 15 - Own address 1 enable
    #[inline(always)]
    pub fn oa1en(&mut self) -> OA1EN_W<OAR1rs> {
        OA1EN_W::new(self, 15)
    }
}
/**I2C own address 1 register

You can [`read`](crate::Reg::read) this register and get [`oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#I2C1:OAR1)*/
pub struct OAR1rs;
impl crate::RegisterSpec for OAR1rs {
    type Ux = u32;
}
///`read()` method returns [`oar1::R`](R) reader structure
impl crate::Readable for OAR1rs {}
///`write(|w| ..)` method takes [`oar1::W`](W) writer structure
impl crate::Writable for OAR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OAR1 to value 0
impl crate::Resettable for OAR1rs {
    const RESET_VALUE: u32 = 0;
}
