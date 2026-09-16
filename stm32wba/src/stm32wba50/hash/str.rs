///Register `STR` reader
pub type R = crate::R<STRrs>;
///Register `STR` writer
pub type W = crate::W<STRrs>;
///Field `NBLW` reader - Number of valid bits in the last word When the last word of the message bit string is written in HASH_DIN register, the hash processor takes only the valid bits specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW\[4:0\] bitfield is written while DCAL is set to 1, the NBLW\[4:0\] bitfield remains unchanged. In other words it is not possible to configure NBLW\[4:0\] and set DCAL at the same time. Reading NBLW\[4:0\] bitfield returns the last value written to NBLW\[4:0\].
pub type NBLW_R = crate::FieldReader;
///Field `NBLW` writer - Number of valid bits in the last word When the last word of the message bit string is written in HASH_DIN register, the hash processor takes only the valid bits specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW\[4:0\] bitfield is written while DCAL is set to 1, the NBLW\[4:0\] bitfield remains unchanged. In other words it is not possible to configure NBLW\[4:0\] and set DCAL at the same time. Reading NBLW\[4:0\] bitfield returns the last value written to NBLW\[4:0\].
pub type NBLW_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DCAL` reader - Digest calculation Writing this bit to 1 starts the message padding, using the previously written value of NBLW\[4:0\], and starts the calculation of the final message digest with all data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0.
pub type DCAL_R = crate::BitReader;
///Field `DCAL` writer - Digest calculation Writing this bit to 1 starts the message padding, using the previously written value of NBLW\[4:0\], and starts the calculation of the final message digest with all data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0.
pub type DCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Number of valid bits in the last word When the last word of the message bit string is written in HASH_DIN register, the hash processor takes only the valid bits specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW\[4:0\] bitfield is written while DCAL is set to 1, the NBLW\[4:0\] bitfield remains unchanged. In other words it is not possible to configure NBLW\[4:0\] and set DCAL at the same time. Reading NBLW\[4:0\] bitfield returns the last value written to NBLW\[4:0\].
    #[inline(always)]
    pub fn nblw(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Digest calculation Writing this bit to 1 starts the message padding, using the previously written value of NBLW\[4:0\], and starts the calculation of the final message digest with all data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0.
    #[inline(always)]
    pub fn dcal(&self) -> DCAL_R {
        DCAL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STR")
            .field("nblw", &self.nblw())
            .field("dcal", &self.dcal())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Number of valid bits in the last word When the last word of the message bit string is written in HASH_DIN register, the hash processor takes only the valid bits specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW\[4:0\] bitfield is written while DCAL is set to 1, the NBLW\[4:0\] bitfield remains unchanged. In other words it is not possible to configure NBLW\[4:0\] and set DCAL at the same time. Reading NBLW\[4:0\] bitfield returns the last value written to NBLW\[4:0\].
    #[inline(always)]
    pub fn nblw(&mut self) -> NBLW_W<'_, STRrs> {
        NBLW_W::new(self, 0)
    }
    ///Bit 8 - Digest calculation Writing this bit to 1 starts the message padding, using the previously written value of NBLW\[4:0\], and starts the calculation of the final message digest with all data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0.
    #[inline(always)]
    pub fn dcal(&mut self) -> DCAL_W<'_, STRrs> {
        DCAL_W::new(self, 8)
    }
}
/**HASH start register

You can [`read`](crate::Reg::read) this register and get [`str::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HASH:STR)*/
pub struct STRrs;
impl crate::RegisterSpec for STRrs {
    type Ux = u32;
}
///`read()` method returns [`str::R`](R) reader structure
impl crate::Readable for STRrs {}
///`write(|w| ..)` method takes [`str::W`](W) writer structure
impl crate::Writable for STRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STR to value 0
impl crate::Resettable for STRrs {}
