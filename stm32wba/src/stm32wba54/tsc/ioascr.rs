///Register `IOASCR` reader
pub type R = crate::R<IOASCRrs>;
///Register `IOASCR` writer
pub type W = crate::W<IOASCRrs>;
/**Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1_IO1 {
    ///0: Gx_IOy analog switch disabled (opened)
    Disabled = 0,
    ///1: Gx_IOy analog switch enabled (closed)
    Enabled = 1,
}
impl From<G1_IO1> for bool {
    #[inline(always)]
    fn from(variant: G1_IO1) -> Self {
        variant as u8 != 0
    }
}
///Field `G_IO1(1-6)` reader - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
pub type G_IO1_R = crate::BitReader<G1_IO1>;
impl G_IO1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> G1_IO1 {
        match self.bits {
            false => G1_IO1::Disabled,
            true => G1_IO1::Enabled,
        }
    }
    ///Gx_IOy analog switch disabled (opened)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == G1_IO1::Disabled
    }
    ///Gx_IOy analog switch enabled (closed)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == G1_IO1::Enabled
    }
}
///Field `G_IO1(1-6)` writer - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
pub type G_IO1_W<'a, REG> = crate::BitWriter<'a, REG, G1_IO1>;
impl<'a, REG> G_IO1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Gx_IOy analog switch disabled (opened)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(G1_IO1::Disabled)
    }
    ///Gx_IOy analog switch enabled (closed)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(G1_IO1::Enabled)
    }
}
///Field `G_IO2(1-6)` reader - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
pub use G_IO1_R as G_IO2_R;
///Field `G_IO3(1-6)` reader - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
pub use G_IO1_R as G_IO3_R;
///Field `G_IO4(1-6)` reader - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
pub use G_IO1_R as G_IO4_R;
///Field `G_IO2(1-6)` writer - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
pub use G_IO1_W as G_IO2_W;
///Field `G_IO3(1-6)` writer - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
pub use G_IO1_W as G_IO3_W;
///Field `G_IO4(1-6)` writer - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
pub use G_IO1_W as G_IO4_W;
impl R {
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO1` field.</div>
    #[inline(always)]
    pub fn g_io1(&self, n: u8) -> G_IO1_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        G_IO1_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g_io1_iter(&self) -> impl Iterator<Item = G_IO1_R> + '_ {
        (0..6).map(move |n| G_IO1_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    ///Bit 0 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g1_io1(&self) -> G_IO1_R {
        G_IO1_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g2_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g3_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g4_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g5_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g6_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO2` field.</div>
    #[inline(always)]
    pub fn g_io2(&self, n: u8) -> G_IO2_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        G_IO2_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g_io2_iter(&self) -> impl Iterator<Item = G_IO2_R> + '_ {
        (0..6).map(move |n| G_IO2_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    ///Bit 1 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g1_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g2_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g3_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g4_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 17 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g5_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g6_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO3` field.</div>
    #[inline(always)]
    pub fn g_io3(&self, n: u8) -> G_IO3_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        G_IO3_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g_io3_iter(&self) -> impl Iterator<Item = G_IO3_R> + '_ {
        (0..6).map(move |n| G_IO3_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
    }
    ///Bit 2 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g1_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g2_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 10 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g3_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g4_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 18 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g5_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 22 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g6_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO4` field.</div>
    #[inline(always)]
    pub fn g_io4(&self, n: u8) -> G_IO4_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        G_IO4_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g_io4_iter(&self) -> impl Iterator<Item = G_IO4_R> + '_ {
        (0..6).map(move |n| G_IO4_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
    }
    ///Bit 3 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g1_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g2_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g3_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g4_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 19 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g5_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g6_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOASCR")
            .field("g1_io1", &self.g1_io1())
            .field("g2_io1", &self.g2_io1())
            .field("g3_io1", &self.g3_io1())
            .field("g4_io1", &self.g4_io1())
            .field("g5_io1", &self.g5_io1())
            .field("g6_io1", &self.g6_io1())
            .field("g1_io2", &self.g1_io2())
            .field("g2_io2", &self.g2_io2())
            .field("g3_io2", &self.g3_io2())
            .field("g4_io2", &self.g4_io2())
            .field("g5_io2", &self.g5_io2())
            .field("g6_io2", &self.g6_io2())
            .field("g1_io3", &self.g1_io3())
            .field("g2_io3", &self.g2_io3())
            .field("g3_io3", &self.g3_io3())
            .field("g4_io3", &self.g4_io3())
            .field("g5_io3", &self.g5_io3())
            .field("g6_io3", &self.g6_io3())
            .field("g1_io4", &self.g1_io4())
            .field("g2_io4", &self.g2_io4())
            .field("g3_io4", &self.g3_io4())
            .field("g4_io4", &self.g4_io4())
            .field("g5_io4", &self.g5_io4())
            .field("g6_io4", &self.g6_io4())
            .finish()
    }
}
impl W {
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO1` field.</div>
    #[inline(always)]
    pub fn g_io1(&mut self, n: u8) -> G_IO1_W<'_, IOASCRrs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        G_IO1_W::new(self, n * 4)
    }
    ///Bit 0 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g1_io1(&mut self) -> G_IO1_W<'_, IOASCRrs> {
        G_IO1_W::new(self, 0)
    }
    ///Bit 4 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g2_io1(&mut self) -> G_IO1_W<'_, IOASCRrs> {
        G_IO1_W::new(self, 4)
    }
    ///Bit 8 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g3_io1(&mut self) -> G_IO1_W<'_, IOASCRrs> {
        G_IO1_W::new(self, 8)
    }
    ///Bit 12 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g4_io1(&mut self) -> G_IO1_W<'_, IOASCRrs> {
        G_IO1_W::new(self, 12)
    }
    ///Bit 16 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g5_io1(&mut self) -> G_IO1_W<'_, IOASCRrs> {
        G_IO1_W::new(self, 16)
    }
    ///Bit 20 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g6_io1(&mut self) -> G_IO1_W<'_, IOASCRrs> {
        G_IO1_W::new(self, 20)
    }
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO2` field.</div>
    #[inline(always)]
    pub fn g_io2(&mut self, n: u8) -> G_IO2_W<'_, IOASCRrs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        G_IO2_W::new(self, n * 4 + 1)
    }
    ///Bit 1 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g1_io2(&mut self) -> G_IO2_W<'_, IOASCRrs> {
        G_IO2_W::new(self, 1)
    }
    ///Bit 5 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g2_io2(&mut self) -> G_IO2_W<'_, IOASCRrs> {
        G_IO2_W::new(self, 5)
    }
    ///Bit 9 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g3_io2(&mut self) -> G_IO2_W<'_, IOASCRrs> {
        G_IO2_W::new(self, 9)
    }
    ///Bit 13 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g4_io2(&mut self) -> G_IO2_W<'_, IOASCRrs> {
        G_IO2_W::new(self, 13)
    }
    ///Bit 17 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g5_io2(&mut self) -> G_IO2_W<'_, IOASCRrs> {
        G_IO2_W::new(self, 17)
    }
    ///Bit 21 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g6_io2(&mut self) -> G_IO2_W<'_, IOASCRrs> {
        G_IO2_W::new(self, 21)
    }
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO3` field.</div>
    #[inline(always)]
    pub fn g_io3(&mut self, n: u8) -> G_IO3_W<'_, IOASCRrs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        G_IO3_W::new(self, n * 4 + 2)
    }
    ///Bit 2 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g1_io3(&mut self) -> G_IO3_W<'_, IOASCRrs> {
        G_IO3_W::new(self, 2)
    }
    ///Bit 6 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g2_io3(&mut self) -> G_IO3_W<'_, IOASCRrs> {
        G_IO3_W::new(self, 6)
    }
    ///Bit 10 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g3_io3(&mut self) -> G_IO3_W<'_, IOASCRrs> {
        G_IO3_W::new(self, 10)
    }
    ///Bit 14 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g4_io3(&mut self) -> G_IO3_W<'_, IOASCRrs> {
        G_IO3_W::new(self, 14)
    }
    ///Bit 18 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g5_io3(&mut self) -> G_IO3_W<'_, IOASCRrs> {
        G_IO3_W::new(self, 18)
    }
    ///Bit 22 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g6_io3(&mut self) -> G_IO3_W<'_, IOASCRrs> {
        G_IO3_W::new(self, 22)
    }
    ///Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO4` field.</div>
    #[inline(always)]
    pub fn g_io4(&mut self, n: u8) -> G_IO4_W<'_, IOASCRrs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        G_IO4_W::new(self, n * 4 + 3)
    }
    ///Bit 3 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g1_io4(&mut self) -> G_IO4_W<'_, IOASCRrs> {
        G_IO4_W::new(self, 3)
    }
    ///Bit 7 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g2_io4(&mut self) -> G_IO4_W<'_, IOASCRrs> {
        G_IO4_W::new(self, 7)
    }
    ///Bit 11 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g3_io4(&mut self) -> G_IO4_W<'_, IOASCRrs> {
        G_IO4_W::new(self, 11)
    }
    ///Bit 15 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g4_io4(&mut self) -> G_IO4_W<'_, IOASCRrs> {
        G_IO4_W::new(self, 15)
    }
    ///Bit 19 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g5_io4(&mut self) -> G_IO4_W<'_, IOASCRrs> {
        G_IO4_W::new(self, 19)
    }
    ///Bit 23 - Gx_IOy analog switch enable These bits are set and cleared by software to enable/disable the Gx_IOy analog switch. Note: These bits control the I/O analog switch whatever the I/O control mode is (even if controlled by standard GPIO registers).
    #[inline(always)]
    pub fn g6_io4(&mut self) -> G_IO4_W<'_, IOASCRrs> {
        G_IO4_W::new(self, 23)
    }
}
/**TSC I/O analog switch control register

You can [`read`](crate::Reg::read) this register and get [`ioascr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioascr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOASCR)*/
pub struct IOASCRrs;
impl crate::RegisterSpec for IOASCRrs {
    type Ux = u32;
}
///`read()` method returns [`ioascr::R`](R) reader structure
impl crate::Readable for IOASCRrs {}
///`write(|w| ..)` method takes [`ioascr::W`](W) writer structure
impl crate::Writable for IOASCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOASCR to value 0
impl crate::Resettable for IOASCRrs {}
