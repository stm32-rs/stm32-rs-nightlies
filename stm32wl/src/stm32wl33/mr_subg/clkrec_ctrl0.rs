///Register `CLKREC_CTRL0` reader
pub type R = crate::R<CLKREC_CTRL0rs>;
///Register `CLKREC_CTRL0` writer
pub type W = crate::W<CLKREC_CTRL0rs>;
///Field `CLKREC_I_GAIN_FAST` reader - Integral fast gain for the clock recovery loop (PLL mode only)
pub type CLKREC_I_GAIN_FAST_R = crate::FieldReader;
///Field `CLKREC_I_GAIN_FAST` writer - Integral fast gain for the clock recovery loop (PLL mode only)
pub type CLKREC_I_GAIN_FAST_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CLKREC_P_GAIN_FAST` reader - Clock recovery fast loop gain (log2)
pub type CLKREC_P_GAIN_FAST_R = crate::FieldReader;
///Field `CLKREC_P_GAIN_FAST` writer - Clock recovery fast loop gain (log2)
pub type CLKREC_P_GAIN_FAST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PSTFLT_LEN` reader - Control the length of the demodulator post-filter
pub type PSTFLT_LEN_R = crate::BitReader;
///Field `PSTFLT_LEN` writer - Control the length of the demodulator post-filter
pub type PSTFLT_LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Integral fast gain for the clock recovery loop (PLL mode only)
    #[inline(always)]
    pub fn clkrec_i_gain_fast(&self) -> CLKREC_I_GAIN_FAST_R {
        CLKREC_I_GAIN_FAST_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Clock recovery fast loop gain (log2)
    #[inline(always)]
    pub fn clkrec_p_gain_fast(&self) -> CLKREC_P_GAIN_FAST_R {
        CLKREC_P_GAIN_FAST_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Control the length of the demodulator post-filter
    #[inline(always)]
    pub fn pstflt_len(&self) -> PSTFLT_LEN_R {
        PSTFLT_LEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKREC_CTRL0")
            .field("clkrec_i_gain_fast", &self.clkrec_i_gain_fast())
            .field("clkrec_p_gain_fast", &self.clkrec_p_gain_fast())
            .field("pstflt_len", &self.pstflt_len())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Integral fast gain for the clock recovery loop (PLL mode only)
    #[inline(always)]
    pub fn clkrec_i_gain_fast(&mut self) -> CLKREC_I_GAIN_FAST_W<'_, CLKREC_CTRL0rs> {
        CLKREC_I_GAIN_FAST_W::new(self, 0)
    }
    ///Bits 4:6 - Clock recovery fast loop gain (log2)
    #[inline(always)]
    pub fn clkrec_p_gain_fast(&mut self) -> CLKREC_P_GAIN_FAST_W<'_, CLKREC_CTRL0rs> {
        CLKREC_P_GAIN_FAST_W::new(self, 4)
    }
    ///Bit 7 - Control the length of the demodulator post-filter
    #[inline(always)]
    pub fn pstflt_len(&mut self) -> PSTFLT_LEN_W<'_, CLKREC_CTRL0rs> {
        PSTFLT_LEN_W::new(self, 7)
    }
}
/**CLKREC_CTRL0 register

You can [`read`](crate::Reg::read) this register and get [`clkrec_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkrec_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:CLKREC_CTRL0)*/
pub struct CLKREC_CTRL0rs;
impl crate::RegisterSpec for CLKREC_CTRL0rs {
    type Ux = u32;
}
///`read()` method returns [`clkrec_ctrl0::R`](R) reader structure
impl crate::Readable for CLKREC_CTRL0rs {}
///`write(|w| ..)` method takes [`clkrec_ctrl0::W`](W) writer structure
impl crate::Writable for CLKREC_CTRL0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLKREC_CTRL0 to value 0xb8
impl crate::Resettable for CLKREC_CTRL0rs {
    const RESET_VALUE: u32 = 0xb8;
}
