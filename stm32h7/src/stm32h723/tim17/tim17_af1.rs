///Register `TIM17_AF1` reader
pub type R = crate::R<TIM17_AF1rs>;
///Register `TIM17_AF1` writer
pub type W = crate::W<TIM17_AF1rs>;
///Field `BKINE` reader - BRK BKIN input enable
pub type BKINE_R = crate::BitReader;
///Field `BKINE` writer - BRK BKIN input enable
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP1E` reader - BRK COMP1 enable
pub type BKCMP1E_R = crate::BitReader;
///Field `BKCMP1E` writer - BRK COMP1 enable
pub type BKCMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP2E` reader - BRK COMP2 enable
pub type BKCMP2E_R = crate::BitReader;
///Field `BKCMP2E` writer - BRK COMP2 enable
pub type BKCMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKDFBK1E` reader - BRK dfsdm1_break\[1\] enable
pub type BKDFBK1E_R = crate::BitReader;
///Field `BKDFBK1E` writer - BRK dfsdm1_break\[1\] enable
pub type BKDFBK1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKINP` reader - BRK BKIN input polarity
pub type BKINP_R = crate::BitReader;
///Field `BKINP` writer - BRK BKIN input polarity
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP1P` reader - BRK COMP1 input polarity
pub type BKCMP1P_R = crate::BitReader;
///Field `BKCMP1P` writer - BRK COMP1 input polarity
pub type BKCMP1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP2P` reader - BRK COMP2 input polarity
pub type BKCMP2P_R = crate::BitReader;
///Field `BKCMP2P` writer - BRK COMP2 input polarity
pub type BKCMP2P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BRK BKIN input enable
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BRK COMP1 enable
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRK COMP2 enable
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - BRK dfsdm1_break\[1\] enable
    #[inline(always)]
    pub fn bkdfbk1e(&self) -> BKDFBK1E_R {
        BKDFBK1E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BRK BKIN input polarity
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BRK COMP1 input polarity
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BRK COMP2 input polarity
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM17_AF1")
            .field("bkine", &self.bkine())
            .field("bkcmp1e", &self.bkcmp1e())
            .field("bkcmp2e", &self.bkcmp2e())
            .field("bkdfbk1e", &self.bkdfbk1e())
            .field("bkinp", &self.bkinp())
            .field("bkcmp1p", &self.bkcmp1p())
            .field("bkcmp2p", &self.bkcmp2p())
            .finish()
    }
}
impl W {
    ///Bit 0 - BRK BKIN input enable
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W<TIM17_AF1rs> {
        BKINE_W::new(self, 0)
    }
    ///Bit 1 - BRK COMP1 enable
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<TIM17_AF1rs> {
        BKCMP1E_W::new(self, 1)
    }
    ///Bit 2 - BRK COMP2 enable
    #[inline(always)]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<TIM17_AF1rs> {
        BKCMP2E_W::new(self, 2)
    }
    ///Bit 8 - BRK dfsdm1_break\[1\] enable
    #[inline(always)]
    pub fn bkdfbk1e(&mut self) -> BKDFBK1E_W<TIM17_AF1rs> {
        BKDFBK1E_W::new(self, 8)
    }
    ///Bit 9 - BRK BKIN input polarity
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W<TIM17_AF1rs> {
        BKINP_W::new(self, 9)
    }
    ///Bit 10 - BRK COMP1 input polarity
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<TIM17_AF1rs> {
        BKCMP1P_W::new(self, 10)
    }
    ///Bit 11 - BRK COMP2 input polarity
    #[inline(always)]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<TIM17_AF1rs> {
        BKCMP2P_W::new(self, 11)
    }
}
/**TIM17 alternate function register 1

You can [`read`](crate::Reg::read) this register and get [`tim17_af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#TIM17:TIM17_AF1)*/
pub struct TIM17_AF1rs;
impl crate::RegisterSpec for TIM17_AF1rs {
    type Ux = u32;
}
///`read()` method returns [`tim17_af1::R`](R) reader structure
impl crate::Readable for TIM17_AF1rs {}
///`write(|w| ..)` method takes [`tim17_af1::W`](W) writer structure
impl crate::Writable for TIM17_AF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM17_AF1 to value 0
impl crate::Resettable for TIM17_AF1rs {}
