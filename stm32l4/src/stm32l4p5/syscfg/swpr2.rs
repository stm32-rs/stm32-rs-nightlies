///Register `SWPR2` reader
pub type R = crate::R<SWPR2rs>;
///Register `SWPR2` writer
pub type W = crate::W<SWPR2rs>;
/**SRAM2 page x write protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P32WP {
    ///0: Write protection of SRAM2 page x is disabled
    Disabled = 0,
    ///1: Write protection of SRAM2 page x is enabled
    Enabled = 1,
}
impl From<P32WP> for bool {
    #[inline(always)]
    fn from(variant: P32WP) -> Self {
        variant as u8 != 0
    }
}
///Field `P32WP` reader - SRAM2 page x write protection
pub type P32WP_R = crate::BitReader<P32WP>;
impl P32WP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P32WP {
        match self.bits {
            false => P32WP::Disabled,
            true => P32WP::Enabled,
        }
    }
    ///Write protection of SRAM2 page x is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P32WP::Disabled
    }
    ///Write protection of SRAM2 page x is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == P32WP::Enabled
    }
}
///Field `P32WP` writer - SRAM2 page x write protection
pub type P32WP_W<'a, REG> = crate::BitWriter<'a, REG, P32WP>;
impl<'a, REG> P32WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write protection of SRAM2 page x is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P32WP::Disabled)
    }
    ///Write protection of SRAM2 page x is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P32WP::Enabled)
    }
}
///Field `P33WP` reader - SRAM2 page x write protection
pub use P32WP_R as P33WP_R;
///Field `P34WP` reader - SRAM2 page x write protection
pub use P32WP_R as P34WP_R;
///Field `P35WP` reader - SRAM2 page x write protection
pub use P32WP_R as P35WP_R;
///Field `P36WP` reader - SRAM2 page x write protection
pub use P32WP_R as P36WP_R;
///Field `P37WP` reader - SRAM2 page x write protection
pub use P32WP_R as P37WP_R;
///Field `P38WP` reader - SRAM2 page x write protection
pub use P32WP_R as P38WP_R;
///Field `P39WP` reader - SRAM2 page x write protection
pub use P32WP_R as P39WP_R;
///Field `P40WP` reader - SRAM2 page x write protection
pub use P32WP_R as P40WP_R;
///Field `P41WP` reader - SRAM2 page x write protection
pub use P32WP_R as P41WP_R;
///Field `P42WP` reader - SRAM2 page x write protection
pub use P32WP_R as P42WP_R;
///Field `P43WP` reader - SRAM2 page x write protection
pub use P32WP_R as P43WP_R;
///Field `P44WP` reader - SRAM2 page x write protection
pub use P32WP_R as P44WP_R;
///Field `P45WP` reader - SRAM2 page x write protection
pub use P32WP_R as P45WP_R;
///Field `P46WP` reader - SRAM2 page x write protection
pub use P32WP_R as P46WP_R;
///Field `P47WP` reader - SRAM2 page x write protection
pub use P32WP_R as P47WP_R;
///Field `P48WP` reader - SRAM2 page x write protection
pub use P32WP_R as P48WP_R;
///Field `P49WP` reader - SRAM2 page x write protection
pub use P32WP_R as P49WP_R;
///Field `P50WP` reader - SRAM2 page x write protection
pub use P32WP_R as P50WP_R;
///Field `P51WP` reader - SRAM2 page x write protection
pub use P32WP_R as P51WP_R;
///Field `P52WP` reader - SRAM2 page x write protection
pub use P32WP_R as P52WP_R;
///Field `P53WP` reader - SRAM2 page x write protection
pub use P32WP_R as P53WP_R;
///Field `P54WP` reader - SRAM2 page x write protection
pub use P32WP_R as P54WP_R;
///Field `P55WP` reader - SRAM2 page x write protection
pub use P32WP_R as P55WP_R;
///Field `P56WP` reader - SRAM2 page x write protection
pub use P32WP_R as P56WP_R;
///Field `P57WP` reader - SRAM2 page x write protection
pub use P32WP_R as P57WP_R;
///Field `P58WP` reader - SRAM2 page x write protection
pub use P32WP_R as P58WP_R;
///Field `P59WP` reader - SRAM2 page x write protection
pub use P32WP_R as P59WP_R;
///Field `P60WP` reader - SRAM2 page x write protection
pub use P32WP_R as P60WP_R;
///Field `P61WP` reader - SRAM2 page x write protection
pub use P32WP_R as P61WP_R;
///Field `P62WP` reader - SRAM2 page x write protection
pub use P32WP_R as P62WP_R;
///Field `P63WP` reader - SRAM2 page x write protection
pub use P32WP_R as P63WP_R;
///Field `P33WP` writer - SRAM2 page x write protection
pub use P32WP_W as P33WP_W;
///Field `P34WP` writer - SRAM2 page x write protection
pub use P32WP_W as P34WP_W;
///Field `P35WP` writer - SRAM2 page x write protection
pub use P32WP_W as P35WP_W;
///Field `P36WP` writer - SRAM2 page x write protection
pub use P32WP_W as P36WP_W;
///Field `P37WP` writer - SRAM2 page x write protection
pub use P32WP_W as P37WP_W;
///Field `P38WP` writer - SRAM2 page x write protection
pub use P32WP_W as P38WP_W;
///Field `P39WP` writer - SRAM2 page x write protection
pub use P32WP_W as P39WP_W;
///Field `P40WP` writer - SRAM2 page x write protection
pub use P32WP_W as P40WP_W;
///Field `P41WP` writer - SRAM2 page x write protection
pub use P32WP_W as P41WP_W;
///Field `P42WP` writer - SRAM2 page x write protection
pub use P32WP_W as P42WP_W;
///Field `P43WP` writer - SRAM2 page x write protection
pub use P32WP_W as P43WP_W;
///Field `P44WP` writer - SRAM2 page x write protection
pub use P32WP_W as P44WP_W;
///Field `P45WP` writer - SRAM2 page x write protection
pub use P32WP_W as P45WP_W;
///Field `P46WP` writer - SRAM2 page x write protection
pub use P32WP_W as P46WP_W;
///Field `P47WP` writer - SRAM2 page x write protection
pub use P32WP_W as P47WP_W;
///Field `P48WP` writer - SRAM2 page x write protection
pub use P32WP_W as P48WP_W;
///Field `P49WP` writer - SRAM2 page x write protection
pub use P32WP_W as P49WP_W;
///Field `P50WP` writer - SRAM2 page x write protection
pub use P32WP_W as P50WP_W;
///Field `P51WP` writer - SRAM2 page x write protection
pub use P32WP_W as P51WP_W;
///Field `P52WP` writer - SRAM2 page x write protection
pub use P32WP_W as P52WP_W;
///Field `P53WP` writer - SRAM2 page x write protection
pub use P32WP_W as P53WP_W;
///Field `P54WP` writer - SRAM2 page x write protection
pub use P32WP_W as P54WP_W;
///Field `P55WP` writer - SRAM2 page x write protection
pub use P32WP_W as P55WP_W;
///Field `P56WP` writer - SRAM2 page x write protection
pub use P32WP_W as P56WP_W;
///Field `P57WP` writer - SRAM2 page x write protection
pub use P32WP_W as P57WP_W;
///Field `P58WP` writer - SRAM2 page x write protection
pub use P32WP_W as P58WP_W;
///Field `P59WP` writer - SRAM2 page x write protection
pub use P32WP_W as P59WP_W;
///Field `P60WP` writer - SRAM2 page x write protection
pub use P32WP_W as P60WP_W;
///Field `P61WP` writer - SRAM2 page x write protection
pub use P32WP_W as P61WP_W;
///Field `P62WP` writer - SRAM2 page x write protection
pub use P32WP_W as P62WP_W;
///Field `P63WP` writer - SRAM2 page x write protection
pub use P32WP_W as P63WP_W;
impl R {
    ///Bit 0 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p32wp(&self) -> P32WP_R {
        P32WP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p33wp(&self) -> P33WP_R {
        P33WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p34wp(&self) -> P34WP_R {
        P34WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p35wp(&self) -> P35WP_R {
        P35WP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p36wp(&self) -> P36WP_R {
        P36WP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p37wp(&self) -> P37WP_R {
        P37WP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p38wp(&self) -> P38WP_R {
        P38WP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p39wp(&self) -> P39WP_R {
        P39WP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p40wp(&self) -> P40WP_R {
        P40WP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p41wp(&self) -> P41WP_R {
        P41WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p42wp(&self) -> P42WP_R {
        P42WP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p43wp(&self) -> P43WP_R {
        P43WP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p44wp(&self) -> P44WP_R {
        P44WP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p45wp(&self) -> P45WP_R {
        P45WP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p46wp(&self) -> P46WP_R {
        P46WP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p47wp(&self) -> P47WP_R {
        P47WP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p48wp(&self) -> P48WP_R {
        P48WP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p49wp(&self) -> P49WP_R {
        P49WP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p50wp(&self) -> P50WP_R {
        P50WP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p51wp(&self) -> P51WP_R {
        P51WP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p52wp(&self) -> P52WP_R {
        P52WP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p53wp(&self) -> P53WP_R {
        P53WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p54wp(&self) -> P54WP_R {
        P54WP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p55wp(&self) -> P55WP_R {
        P55WP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p56wp(&self) -> P56WP_R {
        P56WP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p57wp(&self) -> P57WP_R {
        P57WP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p58wp(&self) -> P58WP_R {
        P58WP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p59wp(&self) -> P59WP_R {
        P59WP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p60wp(&self) -> P60WP_R {
        P60WP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p61wp(&self) -> P61WP_R {
        P61WP_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p62wp(&self) -> P62WP_R {
        P62WP_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p63wp(&self) -> P63WP_R {
        P63WP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWPR2")
            .field("p32wp", &self.p32wp())
            .field("p63wp", &self.p63wp())
            .field("p62wp", &self.p62wp())
            .field("p61wp", &self.p61wp())
            .field("p60wp", &self.p60wp())
            .field("p59wp", &self.p59wp())
            .field("p58wp", &self.p58wp())
            .field("p57wp", &self.p57wp())
            .field("p56wp", &self.p56wp())
            .field("p55wp", &self.p55wp())
            .field("p54wp", &self.p54wp())
            .field("p53wp", &self.p53wp())
            .field("p52wp", &self.p52wp())
            .field("p51wp", &self.p51wp())
            .field("p50wp", &self.p50wp())
            .field("p49wp", &self.p49wp())
            .field("p48wp", &self.p48wp())
            .field("p47wp", &self.p47wp())
            .field("p46wp", &self.p46wp())
            .field("p45wp", &self.p45wp())
            .field("p44wp", &self.p44wp())
            .field("p43wp", &self.p43wp())
            .field("p42wp", &self.p42wp())
            .field("p41wp", &self.p41wp())
            .field("p40wp", &self.p40wp())
            .field("p39wp", &self.p39wp())
            .field("p38wp", &self.p38wp())
            .field("p37wp", &self.p37wp())
            .field("p36wp", &self.p36wp())
            .field("p35wp", &self.p35wp())
            .field("p34wp", &self.p34wp())
            .field("p33wp", &self.p33wp())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p32wp(&mut self) -> P32WP_W<'_, SWPR2rs> {
        P32WP_W::new(self, 0)
    }
    ///Bit 1 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p33wp(&mut self) -> P33WP_W<'_, SWPR2rs> {
        P33WP_W::new(self, 1)
    }
    ///Bit 2 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p34wp(&mut self) -> P34WP_W<'_, SWPR2rs> {
        P34WP_W::new(self, 2)
    }
    ///Bit 3 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p35wp(&mut self) -> P35WP_W<'_, SWPR2rs> {
        P35WP_W::new(self, 3)
    }
    ///Bit 4 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p36wp(&mut self) -> P36WP_W<'_, SWPR2rs> {
        P36WP_W::new(self, 4)
    }
    ///Bit 5 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p37wp(&mut self) -> P37WP_W<'_, SWPR2rs> {
        P37WP_W::new(self, 5)
    }
    ///Bit 6 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p38wp(&mut self) -> P38WP_W<'_, SWPR2rs> {
        P38WP_W::new(self, 6)
    }
    ///Bit 7 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p39wp(&mut self) -> P39WP_W<'_, SWPR2rs> {
        P39WP_W::new(self, 7)
    }
    ///Bit 8 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p40wp(&mut self) -> P40WP_W<'_, SWPR2rs> {
        P40WP_W::new(self, 8)
    }
    ///Bit 9 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p41wp(&mut self) -> P41WP_W<'_, SWPR2rs> {
        P41WP_W::new(self, 9)
    }
    ///Bit 10 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p42wp(&mut self) -> P42WP_W<'_, SWPR2rs> {
        P42WP_W::new(self, 10)
    }
    ///Bit 11 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p43wp(&mut self) -> P43WP_W<'_, SWPR2rs> {
        P43WP_W::new(self, 11)
    }
    ///Bit 12 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p44wp(&mut self) -> P44WP_W<'_, SWPR2rs> {
        P44WP_W::new(self, 12)
    }
    ///Bit 13 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p45wp(&mut self) -> P45WP_W<'_, SWPR2rs> {
        P45WP_W::new(self, 13)
    }
    ///Bit 14 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p46wp(&mut self) -> P46WP_W<'_, SWPR2rs> {
        P46WP_W::new(self, 14)
    }
    ///Bit 15 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p47wp(&mut self) -> P47WP_W<'_, SWPR2rs> {
        P47WP_W::new(self, 15)
    }
    ///Bit 16 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p48wp(&mut self) -> P48WP_W<'_, SWPR2rs> {
        P48WP_W::new(self, 16)
    }
    ///Bit 17 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p49wp(&mut self) -> P49WP_W<'_, SWPR2rs> {
        P49WP_W::new(self, 17)
    }
    ///Bit 18 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p50wp(&mut self) -> P50WP_W<'_, SWPR2rs> {
        P50WP_W::new(self, 18)
    }
    ///Bit 19 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p51wp(&mut self) -> P51WP_W<'_, SWPR2rs> {
        P51WP_W::new(self, 19)
    }
    ///Bit 20 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p52wp(&mut self) -> P52WP_W<'_, SWPR2rs> {
        P52WP_W::new(self, 20)
    }
    ///Bit 21 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p53wp(&mut self) -> P53WP_W<'_, SWPR2rs> {
        P53WP_W::new(self, 21)
    }
    ///Bit 22 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p54wp(&mut self) -> P54WP_W<'_, SWPR2rs> {
        P54WP_W::new(self, 22)
    }
    ///Bit 23 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p55wp(&mut self) -> P55WP_W<'_, SWPR2rs> {
        P55WP_W::new(self, 23)
    }
    ///Bit 24 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p56wp(&mut self) -> P56WP_W<'_, SWPR2rs> {
        P56WP_W::new(self, 24)
    }
    ///Bit 25 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p57wp(&mut self) -> P57WP_W<'_, SWPR2rs> {
        P57WP_W::new(self, 25)
    }
    ///Bit 26 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p58wp(&mut self) -> P58WP_W<'_, SWPR2rs> {
        P58WP_W::new(self, 26)
    }
    ///Bit 27 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p59wp(&mut self) -> P59WP_W<'_, SWPR2rs> {
        P59WP_W::new(self, 27)
    }
    ///Bit 28 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p60wp(&mut self) -> P60WP_W<'_, SWPR2rs> {
        P60WP_W::new(self, 28)
    }
    ///Bit 29 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p61wp(&mut self) -> P61WP_W<'_, SWPR2rs> {
        P61WP_W::new(self, 29)
    }
    ///Bit 30 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p62wp(&mut self) -> P62WP_W<'_, SWPR2rs> {
        P62WP_W::new(self, 30)
    }
    ///Bit 31 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p63wp(&mut self) -> P63WP_W<'_, SWPR2rs> {
        P63WP_W::new(self, 31)
    }
}
/**write protection register 2

You can [`read`](crate::Reg::read) this register and get [`swpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#SYSCFG:SWPR2)*/
pub struct SWPR2rs;
impl crate::RegisterSpec for SWPR2rs {
    type Ux = u32;
}
///`read()` method returns [`swpr2::R`](R) reader structure
impl crate::Readable for SWPR2rs {}
///`write(|w| ..)` method takes [`swpr2::W`](W) writer structure
impl crate::Writable for SWPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWPR2 to value 0
impl crate::Resettable for SWPR2rs {}
