///Register `OFR4` reader
pub type R = crate::R<OFR4rs>;
///Register `OFR4` writer
pub type W = crate::W<OFR4rs>;
///Field `OFFSET4` reader - OFFSET4
pub type OFFSET4_R = crate::FieldReader<u16>;
///Field `OFFSET4` writer - OFFSET4
pub type OFFSET4_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `OFFSET4_CH` reader - OFFSET4_CH
pub type OFFSET4_CH_R = crate::FieldReader;
///Field `OFFSET4_CH` writer - OFFSET4_CH
pub type OFFSET4_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**OFFSET4_EN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSET4_EN {
    ///0: This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    Disabled = 0,
    ///1: This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    Enabled = 1,
}
impl From<OFFSET4_EN> for bool {
    #[inline(always)]
    fn from(variant: OFFSET4_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OFFSET4_EN` reader - OFFSET4_EN
pub type OFFSET4_EN_R = crate::BitReader<OFFSET4_EN>;
impl OFFSET4_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFFSET4_EN {
        match self.bits {
            false => OFFSET4_EN::Disabled,
            true => OFFSET4_EN::Enabled,
        }
    }
    ///This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET4_EN::Disabled
    }
    ///This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET4_EN::Enabled
    }
}
///Field `OFFSET4_EN` writer - OFFSET4_EN
pub type OFFSET4_EN_W<'a, REG> = crate::BitWriter<'a, REG, OFFSET4_EN>;
impl<'a, REG> OFFSET4_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET4_EN::Disabled)
    }
    ///This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET4_EN::Enabled)
    }
}
impl R {
    ///Bits 0:11 - OFFSET4
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 26:30 - OFFSET4_CH
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - OFFSET4_EN
    #[inline(always)]
    pub fn offset4_en(&self) -> OFFSET4_EN_R {
        OFFSET4_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR4")
            .field("offset4_en", &self.offset4_en())
            .field("offset4_ch", &self.offset4_ch())
            .field("offset4", &self.offset4())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - OFFSET4
    #[inline(always)]
    pub fn offset4(&mut self) -> OFFSET4_W<OFR4rs> {
        OFFSET4_W::new(self, 0)
    }
    ///Bits 26:30 - OFFSET4_CH
    #[inline(always)]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W<OFR4rs> {
        OFFSET4_CH_W::new(self, 26)
    }
    ///Bit 31 - OFFSET4_EN
    #[inline(always)]
    pub fn offset4_en(&mut self) -> OFFSET4_EN_W<OFR4rs> {
        OFFSET4_EN_W::new(self, 31)
    }
}
/**offset register 4

You can [`read`](crate::Reg::read) this register and get [`ofr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#ADC1:OFR4)*/
pub struct OFR4rs;
impl crate::RegisterSpec for OFR4rs {
    type Ux = u32;
}
///`read()` method returns [`ofr4::R`](R) reader structure
impl crate::Readable for OFR4rs {}
///`write(|w| ..)` method takes [`ofr4::W`](W) writer structure
impl crate::Writable for OFR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OFR4 to value 0
impl crate::Resettable for OFR4rs {
    const RESET_VALUE: u32 = 0;
}
