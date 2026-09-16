///Register `CLKREC_CTRL1` reader
pub type R = crate::R<CLKREC_CTRL1rs>;
///Register `CLKREC_CTRL1` writer
pub type W = crate::W<CLKREC_CTRL1rs>;
///Field `CLKREC_I_GAIN_SLOW` reader - Integral slow gain for the clock recovery loop (PLL mode only)
pub type CLKREC_I_GAIN_SLOW_R = crate::FieldReader;
///Field `CLKREC_I_GAIN_SLOW` writer - Integral slow gain for the clock recovery loop (PLL mode only)
pub type CLKREC_I_GAIN_SLOW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CLKREC_P_GAIN_SLOW` reader - Clock recovery slow loop gain (log2)
pub type CLKREC_P_GAIN_SLOW_R = crate::FieldReader;
///Field `CLKREC_P_GAIN_SLOW` writer - Clock recovery slow loop gain (log2)
pub type CLKREC_P_GAIN_SLOW_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CLKREC_ALGO_SEL` reader - Symbol timing recovery algorithm selection
pub type CLKREC_ALGO_SEL_R = crate::BitReader;
///Field `CLKREC_ALGO_SEL` writer - Symbol timing recovery algorithm selection
pub type CLKREC_ALGO_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Integral slow gain for the clock recovery loop (PLL mode only)
    #[inline(always)]
    pub fn clkrec_i_gain_slow(&self) -> CLKREC_I_GAIN_SLOW_R {
        CLKREC_I_GAIN_SLOW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Clock recovery slow loop gain (log2)
    #[inline(always)]
    pub fn clkrec_p_gain_slow(&self) -> CLKREC_P_GAIN_SLOW_R {
        CLKREC_P_GAIN_SLOW_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Symbol timing recovery algorithm selection
    #[inline(always)]
    pub fn clkrec_algo_sel(&self) -> CLKREC_ALGO_SEL_R {
        CLKREC_ALGO_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKREC_CTRL1")
            .field("clkrec_i_gain_slow", &self.clkrec_i_gain_slow())
            .field("clkrec_p_gain_slow", &self.clkrec_p_gain_slow())
            .field("clkrec_algo_sel", &self.clkrec_algo_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Integral slow gain for the clock recovery loop (PLL mode only)
    #[inline(always)]
    pub fn clkrec_i_gain_slow(&mut self) -> CLKREC_I_GAIN_SLOW_W<'_, CLKREC_CTRL1rs> {
        CLKREC_I_GAIN_SLOW_W::new(self, 0)
    }
    ///Bits 4:6 - Clock recovery slow loop gain (log2)
    #[inline(always)]
    pub fn clkrec_p_gain_slow(&mut self) -> CLKREC_P_GAIN_SLOW_W<'_, CLKREC_CTRL1rs> {
        CLKREC_P_GAIN_SLOW_W::new(self, 4)
    }
    ///Bit 7 - Symbol timing recovery algorithm selection
    #[inline(always)]
    pub fn clkrec_algo_sel(&mut self) -> CLKREC_ALGO_SEL_W<'_, CLKREC_CTRL1rs> {
        CLKREC_ALGO_SEL_W::new(self, 7)
    }
}
/**CLKREC_CTRL1 register

You can [`read`](crate::Reg::read) this register and get [`clkrec_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkrec_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:CLKREC_CTRL1)*/
pub struct CLKREC_CTRL1rs;
impl crate::RegisterSpec for CLKREC_CTRL1rs {
    type Ux = u32;
}
///`read()` method returns [`clkrec_ctrl1::R`](R) reader structure
impl crate::Readable for CLKREC_CTRL1rs {}
///`write(|w| ..)` method takes [`clkrec_ctrl1::W`](W) writer structure
impl crate::Writable for CLKREC_CTRL1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLKREC_CTRL1 to value 0x5c
impl crate::Resettable for CLKREC_CTRL1rs {
    const RESET_VALUE: u32 = 0x5c;
}
