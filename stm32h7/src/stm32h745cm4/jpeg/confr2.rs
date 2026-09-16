///Register `CONFR2` reader
pub type R = crate::R<CONFR2rs>;
///Register `CONFR2` writer
pub type W = crate::W<CONFR2rs>;
///Field `NMCU` reader - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated.
pub type NMCU_R = crate::FieldReader<u32>;
///Field `NMCU` writer - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated.
pub type NMCU_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated.
    #[inline(always)]
    pub fn nmcu(&self) -> NMCU_R {
        NMCU_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFR2")
            .field("nmcu", &self.nmcu())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated.
    #[inline(always)]
    pub fn nmcu(&mut self) -> NMCU_W<'_, CONFR2rs> {
        NMCU_W::new(self, 0)
    }
}
/**JPEG codec configuration register 2

You can [`read`](crate::Reg::read) this register and get [`confr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#JPEG:CONFR2)*/
pub struct CONFR2rs;
impl crate::RegisterSpec for CONFR2rs {
    type Ux = u32;
}
///`read()` method returns [`confr2::R`](R) reader structure
impl crate::Readable for CONFR2rs {}
///`write(|w| ..)` method takes [`confr2::W`](W) writer structure
impl crate::Writable for CONFR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFR2 to value 0
impl crate::Resettable for CONFR2rs {}
