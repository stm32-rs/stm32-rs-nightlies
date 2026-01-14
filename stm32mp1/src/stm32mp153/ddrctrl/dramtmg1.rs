///Register `DRAMTMG1` reader
pub type R = crate::R<DRAMTMG1rs>;
///Register `DRAMTMG1` writer
pub type W = crate::W<DRAMTMG1rs>;
///Field `T_RC` reader - T_RC
pub type T_RC_R = crate::FieldReader;
///Field `T_RC` writer - T_RC
pub type T_RC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `RD2PRE` reader - RD2PRE
pub type RD2PRE_R = crate::FieldReader;
///Field `RD2PRE` writer - RD2PRE
pub type RD2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `T_XP` reader - T_XP
pub type T_XP_R = crate::FieldReader;
///Field `T_XP` writer - T_XP
pub type T_XP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:6 - T_RC
    #[inline(always)]
    pub fn t_rc(&self) -> T_RC_R {
        T_RC_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:13 - RD2PRE
    #[inline(always)]
    pub fn rd2pre(&self) -> RD2PRE_R {
        RD2PRE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:20 - T_XP
    #[inline(always)]
    pub fn t_xp(&self) -> T_XP_R {
        T_XP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRAMTMG1")
            .field("t_rc", &self.t_rc())
            .field("rd2pre", &self.rd2pre())
            .field("t_xp", &self.t_xp())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - T_RC
    #[inline(always)]
    pub fn t_rc(&mut self) -> T_RC_W<'_, DRAMTMG1rs> {
        T_RC_W::new(self, 0)
    }
    ///Bits 8:13 - RD2PRE
    #[inline(always)]
    pub fn rd2pre(&mut self) -> RD2PRE_W<'_, DRAMTMG1rs> {
        RD2PRE_W::new(self, 8)
    }
    ///Bits 16:20 - T_XP
    #[inline(always)]
    pub fn t_xp(&mut self) -> T_XP_W<'_, DRAMTMG1rs> {
        T_XP_W::new(self, 16)
    }
}
/**DDRCTRL SDRAM timing register 1

You can [`read`](crate::Reg::read) this register and get [`dramtmg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DRAMTMG1)*/
pub struct DRAMTMG1rs;
impl crate::RegisterSpec for DRAMTMG1rs {
    type Ux = u32;
}
///`read()` method returns [`dramtmg1::R`](R) reader structure
impl crate::Readable for DRAMTMG1rs {}
///`write(|w| ..)` method takes [`dramtmg1::W`](W) writer structure
impl crate::Writable for DRAMTMG1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DRAMTMG1 to value 0x0008_0414
impl crate::Resettable for DRAMTMG1rs {
    const RESET_VALUE: u32 = 0x0008_0414;
}
