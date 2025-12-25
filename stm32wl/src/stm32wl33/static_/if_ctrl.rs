///Register `IF_CTRL` reader
pub type R = crate::R<IF_CTRLrs>;
///Register `IF_CTRL` writer
pub type W = crate::W<IF_CTRLrs>;
///Field `IF_OFFSET_DIG` reader - Intermediate frequency setting for the digital shift-to-baseband circuits (default: 300 kHz)
pub type IF_OFFSET_DIG_R = crate::FieldReader<u16>;
///Field `IF_OFFSET_DIG` writer - Intermediate frequency setting for the digital shift-to-baseband circuits (default: 300 kHz)
pub type IF_OFFSET_DIG_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `IF_OFFSET_ANA` reader - Intermediate frequency setting for the synthesizer configuration (default: 300 kHz).
pub type IF_OFFSET_ANA_R = crate::FieldReader<u16>;
///Field `IF_OFFSET_ANA` writer - Intermediate frequency setting for the synthesizer configuration (default: 300 kHz).
pub type IF_OFFSET_ANA_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `IF_MODE` reader - Select the cutoff frequency of the AAF for the analog RFSUBG IP
pub type IF_MODE_R = crate::BitReader;
///Field `IF_MODE` writer - Select the cutoff frequency of the AAF for the analog RFSUBG IP
pub type IF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:12 - Intermediate frequency setting for the digital shift-to-baseband circuits (default: 300 kHz)
    #[inline(always)]
    pub fn if_offset_dig(&self) -> IF_OFFSET_DIG_R {
        IF_OFFSET_DIG_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - Intermediate frequency setting for the synthesizer configuration (default: 300 kHz).
    #[inline(always)]
    pub fn if_offset_ana(&self) -> IF_OFFSET_ANA_R {
        IF_OFFSET_ANA_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    ///Bit 31 - Select the cutoff frequency of the AAF for the analog RFSUBG IP
    #[inline(always)]
    pub fn if_mode(&self) -> IF_MODE_R {
        IF_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF_CTRL")
            .field("if_offset_dig", &self.if_offset_dig())
            .field("if_offset_ana", &self.if_offset_ana())
            .field("if_mode", &self.if_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - Intermediate frequency setting for the digital shift-to-baseband circuits (default: 300 kHz)
    #[inline(always)]
    pub fn if_offset_dig(&mut self) -> IF_OFFSET_DIG_W<'_, IF_CTRLrs> {
        IF_OFFSET_DIG_W::new(self, 0)
    }
    ///Bits 16:28 - Intermediate frequency setting for the synthesizer configuration (default: 300 kHz).
    #[inline(always)]
    pub fn if_offset_ana(&mut self) -> IF_OFFSET_ANA_W<'_, IF_CTRLrs> {
        IF_OFFSET_ANA_W::new(self, 16)
    }
    ///Bit 31 - Select the cutoff frequency of the AAF for the analog RFSUBG IP
    #[inline(always)]
    pub fn if_mode(&mut self) -> IF_MODE_W<'_, IF_CTRLrs> {
        IF_MODE_W::new(self, 31)
    }
}
/**IF_CTRL register

You can [`read`](crate::Reg::read) this register and get [`if_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:IF_CTRL)*/
pub struct IF_CTRLrs;
impl crate::RegisterSpec for IF_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`if_ctrl::R`](R) reader structure
impl crate::Readable for IF_CTRLrs {}
///`write(|w| ..)` method takes [`if_ctrl::W`](W) writer structure
impl crate::Writable for IF_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IF_CTRL to value 0x04cd_04cd
impl crate::Resettable for IF_CTRLrs {
    const RESET_VALUE: u32 = 0x04cd_04cd;
}
