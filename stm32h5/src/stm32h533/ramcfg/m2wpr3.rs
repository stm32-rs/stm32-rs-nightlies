///Register `M2WPR3` reader
pub type R = crate::R<M2WPR3rs>;
///Register `M2WPR3` writer
pub type W = crate::W<M2WPR3rs>;
///Field `P64WP` reader - SRAM2 1-Kbyte page y write protection
pub type P64WP_R = crate::BitReader;
///Field `P64WP` writer - SRAM2 1-Kbyte page y write protection
pub type P64WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P65WP` reader - SRAM2 1-Kbyte page y write protection
pub type P65WP_R = crate::BitReader;
///Field `P65WP` writer - SRAM2 1-Kbyte page y write protection
pub type P65WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P66WP` reader - SRAM2 1-Kbyte page y write protection
pub type P66WP_R = crate::BitReader;
///Field `P66WP` writer - SRAM2 1-Kbyte page y write protection
pub type P66WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P67WP` reader - SRAM2 1-Kbyte page y write protection
pub type P67WP_R = crate::BitReader;
///Field `P67WP` writer - SRAM2 1-Kbyte page y write protection
pub type P67WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P68WP` reader - SRAM2 1-Kbyte page y write protection
pub type P68WP_R = crate::BitReader;
///Field `P68WP` writer - SRAM2 1-Kbyte page y write protection
pub type P68WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P69WP` reader - SRAM2 1-Kbyte page y write protection
pub type P69WP_R = crate::BitReader;
///Field `P69WP` writer - SRAM2 1-Kbyte page y write protection
pub type P69WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P70WP` reader - SRAM2 1-Kbyte page y write protection
pub type P70WP_R = crate::BitReader;
///Field `P70WP` writer - SRAM2 1-Kbyte page y write protection
pub type P70WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P71WP` reader - SRAM2 1-Kbyte page y write protection
pub type P71WP_R = crate::BitReader;
///Field `P71WP` writer - SRAM2 1-Kbyte page y write protection
pub type P71WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P72WP` reader - SRAM2 1-Kbyte page y write protection
pub type P72WP_R = crate::BitReader;
///Field `P72WP` writer - SRAM2 1-Kbyte page y write protection
pub type P72WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P73WP` reader - SRAM2 1-Kbyte page y write protection
pub type P73WP_R = crate::BitReader;
///Field `P73WP` writer - SRAM2 1-Kbyte page y write protection
pub type P73WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P74WP` reader - SRAM2 1-Kbyte page y write protection
pub type P74WP_R = crate::BitReader;
///Field `P74WP` writer - SRAM2 1-Kbyte page y write protection
pub type P74WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P75WP` reader - SRAM2 1-Kbyte page y write protection
pub type P75WP_R = crate::BitReader;
///Field `P75WP` writer - SRAM2 1-Kbyte page y write protection
pub type P75WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P76WP` reader - SRAM2 1-Kbyte page y write protection
pub type P76WP_R = crate::BitReader;
///Field `P76WP` writer - SRAM2 1-Kbyte page y write protection
pub type P76WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P77WP` reader - SRAM2 1-Kbyte page y write protection
pub type P77WP_R = crate::BitReader;
///Field `P77WP` writer - SRAM2 1-Kbyte page y write protection
pub type P77WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P78WP` reader - SRAM2 1-Kbyte page y write protection
pub type P78WP_R = crate::BitReader;
///Field `P78WP` writer - SRAM2 1-Kbyte page y write protection
pub type P78WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P79WP` reader - SRAM2 1-Kbyte page y write protection
pub type P79WP_R = crate::BitReader;
///Field `P79WP` writer - SRAM2 1-Kbyte page y write protection
pub type P79WP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p64wp(&self) -> P64WP_R {
        P64WP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p65wp(&self) -> P65WP_R {
        P65WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p66wp(&self) -> P66WP_R {
        P66WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p67wp(&self) -> P67WP_R {
        P67WP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p68wp(&self) -> P68WP_R {
        P68WP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p69wp(&self) -> P69WP_R {
        P69WP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p70wp(&self) -> P70WP_R {
        P70WP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p71wp(&self) -> P71WP_R {
        P71WP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p72wp(&self) -> P72WP_R {
        P72WP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p73wp(&self) -> P73WP_R {
        P73WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p74wp(&self) -> P74WP_R {
        P74WP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p75wp(&self) -> P75WP_R {
        P75WP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p76wp(&self) -> P76WP_R {
        P76WP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p77wp(&self) -> P77WP_R {
        P77WP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p78wp(&self) -> P78WP_R {
        P78WP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p79wp(&self) -> P79WP_R {
        P79WP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2WPR3")
            .field("p64wp", &self.p64wp())
            .field("p65wp", &self.p65wp())
            .field("p66wp", &self.p66wp())
            .field("p67wp", &self.p67wp())
            .field("p68wp", &self.p68wp())
            .field("p69wp", &self.p69wp())
            .field("p70wp", &self.p70wp())
            .field("p71wp", &self.p71wp())
            .field("p72wp", &self.p72wp())
            .field("p73wp", &self.p73wp())
            .field("p74wp", &self.p74wp())
            .field("p75wp", &self.p75wp())
            .field("p76wp", &self.p76wp())
            .field("p77wp", &self.p77wp())
            .field("p78wp", &self.p78wp())
            .field("p79wp", &self.p79wp())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p64wp(&mut self) -> P64WP_W<'_, M2WPR3rs> {
        P64WP_W::new(self, 0)
    }
    ///Bit 1 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p65wp(&mut self) -> P65WP_W<'_, M2WPR3rs> {
        P65WP_W::new(self, 1)
    }
    ///Bit 2 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p66wp(&mut self) -> P66WP_W<'_, M2WPR3rs> {
        P66WP_W::new(self, 2)
    }
    ///Bit 3 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p67wp(&mut self) -> P67WP_W<'_, M2WPR3rs> {
        P67WP_W::new(self, 3)
    }
    ///Bit 4 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p68wp(&mut self) -> P68WP_W<'_, M2WPR3rs> {
        P68WP_W::new(self, 4)
    }
    ///Bit 5 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p69wp(&mut self) -> P69WP_W<'_, M2WPR3rs> {
        P69WP_W::new(self, 5)
    }
    ///Bit 6 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p70wp(&mut self) -> P70WP_W<'_, M2WPR3rs> {
        P70WP_W::new(self, 6)
    }
    ///Bit 7 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p71wp(&mut self) -> P71WP_W<'_, M2WPR3rs> {
        P71WP_W::new(self, 7)
    }
    ///Bit 8 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p72wp(&mut self) -> P72WP_W<'_, M2WPR3rs> {
        P72WP_W::new(self, 8)
    }
    ///Bit 9 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p73wp(&mut self) -> P73WP_W<'_, M2WPR3rs> {
        P73WP_W::new(self, 9)
    }
    ///Bit 10 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p74wp(&mut self) -> P74WP_W<'_, M2WPR3rs> {
        P74WP_W::new(self, 10)
    }
    ///Bit 11 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p75wp(&mut self) -> P75WP_W<'_, M2WPR3rs> {
        P75WP_W::new(self, 11)
    }
    ///Bit 12 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p76wp(&mut self) -> P76WP_W<'_, M2WPR3rs> {
        P76WP_W::new(self, 12)
    }
    ///Bit 13 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p77wp(&mut self) -> P77WP_W<'_, M2WPR3rs> {
        P77WP_W::new(self, 13)
    }
    ///Bit 14 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p78wp(&mut self) -> P78WP_W<'_, M2WPR3rs> {
        P78WP_W::new(self, 14)
    }
    ///Bit 15 - SRAM2 1-Kbyte page y write protection
    #[inline(always)]
    pub fn p79wp(&mut self) -> P79WP_W<'_, M2WPR3rs> {
        P79WP_W::new(self, 15)
    }
}
/**RAMCFG memory 2 write protection register 3

You can [`read`](crate::Reg::read) this register and get [`m2wpr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2wpr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RAMCFG:M2WPR3)*/
pub struct M2WPR3rs;
impl crate::RegisterSpec for M2WPR3rs {
    type Ux = u32;
}
///`read()` method returns [`m2wpr3::R`](R) reader structure
impl crate::Readable for M2WPR3rs {}
///`write(|w| ..)` method takes [`m2wpr3::W`](W) writer structure
impl crate::Writable for M2WPR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M2WPR3 to value 0
impl crate::Resettable for M2WPR3rs {}
