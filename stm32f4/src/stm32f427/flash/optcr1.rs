///Register `OPTCR1` reader
pub type R = crate::R<OPTCR1rs>;
///Register `OPTCR1` writer
pub type W = crate::W<OPTCR1rs>;
/**Not write protect

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_WRP0 {
    ///0: Write protection active on sector %s (bank 2)
    Active = 0,
    ///1: Write protection inactive on sector %s (bank 2)
    Inactive = 1,
}
impl From<N_WRP0> for bool {
    #[inline(always)]
    fn from(variant: N_WRP0) -> Self {
        variant as u8 != 0
    }
}
///Field `nWRP(0-11)` reader - Not write protect
pub type N_WRP_R = crate::BitReader<N_WRP0>;
impl N_WRP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_WRP0 {
        match self.bits {
            false => N_WRP0::Active,
            true => N_WRP0::Inactive,
        }
    }
    ///Write protection active on sector %s (bank 2)
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == N_WRP0::Active
    }
    ///Write protection inactive on sector %s (bank 2)
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == N_WRP0::Inactive
    }
}
///Field `nWRP(0-11)` writer - Not write protect
pub type N_WRP_W<'a, REG> = crate::BitWriter<'a, REG, N_WRP0>;
impl<'a, REG> N_WRP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write protection active on sector %s (bank 2)
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(N_WRP0::Active)
    }
    ///Write protection inactive on sector %s (bank 2)
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(N_WRP0::Inactive)
    }
}
impl R {
    ///Not write protect
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `nWRP0` field.</div>
    #[inline(always)]
    pub fn n_wrp(&self, n: u8) -> N_WRP_R {
        #[allow(clippy::no_effect)]
        [(); 12][n as usize];
        N_WRP_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Not write protect
    #[inline(always)]
    pub fn n_wrp_iter(&self) -> impl Iterator<Item = N_WRP_R> + '_ {
        (0..12).map(move |n| N_WRP_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - Not write protect
    #[inline(always)]
    pub fn n_wrp0(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Not write protect
    #[inline(always)]
    pub fn n_wrp1(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Not write protect
    #[inline(always)]
    pub fn n_wrp2(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Not write protect
    #[inline(always)]
    pub fn n_wrp3(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Not write protect
    #[inline(always)]
    pub fn n_wrp4(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Not write protect
    #[inline(always)]
    pub fn n_wrp5(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Not write protect
    #[inline(always)]
    pub fn n_wrp6(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Not write protect
    #[inline(always)]
    pub fn n_wrp7(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Not write protect
    #[inline(always)]
    pub fn n_wrp8(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Not write protect
    #[inline(always)]
    pub fn n_wrp9(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Not write protect
    #[inline(always)]
    pub fn n_wrp10(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Not write protect
    #[inline(always)]
    pub fn n_wrp11(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR1")
            .field("n_wrp0", &self.n_wrp0())
            .field("n_wrp1", &self.n_wrp1())
            .field("n_wrp2", &self.n_wrp2())
            .field("n_wrp3", &self.n_wrp3())
            .field("n_wrp4", &self.n_wrp4())
            .field("n_wrp5", &self.n_wrp5())
            .field("n_wrp6", &self.n_wrp6())
            .field("n_wrp7", &self.n_wrp7())
            .field("n_wrp8", &self.n_wrp8())
            .field("n_wrp9", &self.n_wrp9())
            .field("n_wrp10", &self.n_wrp10())
            .field("n_wrp11", &self.n_wrp11())
            .finish()
    }
}
impl W {
    ///Not write protect
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `nWRP0` field.</div>
    #[inline(always)]
    pub fn n_wrp(&mut self, n: u8) -> N_WRP_W<'_, OPTCR1rs> {
        #[allow(clippy::no_effect)]
        [(); 12][n as usize];
        N_WRP_W::new(self, n + 16)
    }
    ///Bit 16 - Not write protect
    #[inline(always)]
    pub fn n_wrp0(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 16)
    }
    ///Bit 17 - Not write protect
    #[inline(always)]
    pub fn n_wrp1(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 17)
    }
    ///Bit 18 - Not write protect
    #[inline(always)]
    pub fn n_wrp2(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 18)
    }
    ///Bit 19 - Not write protect
    #[inline(always)]
    pub fn n_wrp3(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 19)
    }
    ///Bit 20 - Not write protect
    #[inline(always)]
    pub fn n_wrp4(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 20)
    }
    ///Bit 21 - Not write protect
    #[inline(always)]
    pub fn n_wrp5(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 21)
    }
    ///Bit 22 - Not write protect
    #[inline(always)]
    pub fn n_wrp6(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 22)
    }
    ///Bit 23 - Not write protect
    #[inline(always)]
    pub fn n_wrp7(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 23)
    }
    ///Bit 24 - Not write protect
    #[inline(always)]
    pub fn n_wrp8(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 24)
    }
    ///Bit 25 - Not write protect
    #[inline(always)]
    pub fn n_wrp9(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 25)
    }
    ///Bit 26 - Not write protect
    #[inline(always)]
    pub fn n_wrp10(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 26)
    }
    ///Bit 27 - Not write protect
    #[inline(always)]
    pub fn n_wrp11(&mut self) -> N_WRP_W<'_, OPTCR1rs> {
        N_WRP_W::new(self, 27)
    }
}
/**Flash option control register 1

You can [`read`](crate::Reg::read) this register and get [`optcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F427.html#FLASH:OPTCR1)*/
pub struct OPTCR1rs;
impl crate::RegisterSpec for OPTCR1rs {
    type Ux = u32;
}
///`read()` method returns [`optcr1::R`](R) reader structure
impl crate::Readable for OPTCR1rs {}
///`write(|w| ..)` method takes [`optcr1::W`](W) writer structure
impl crate::Writable for OPTCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTCR1 to value 0x0fff_0000
impl crate::Resettable for OPTCR1rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
