///Register `AWSR` reader
pub type R = crate::R<AWSRrs>;
///Register `AWSR` writer
pub type W = crate::W<AWSRrs>;
/**Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWLTF0 {
    ///0: No low threshold error
    NoError = 0,
    ///1: A low threshold error on channel y
    Error = 1,
}
impl From<AWLTF0> for bool {
    #[inline(always)]
    fn from(variant: AWLTF0) -> Self {
        variant as u8 != 0
    }
}
///Field `AWLTF(0-7)` reader - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
pub type AWLTF_R = crate::BitReader<AWLTF0>;
impl AWLTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWLTF0 {
        match self.bits {
            false => AWLTF0::NoError,
            true => AWLTF0::Error,
        }
    }
    ///No low threshold error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AWLTF0::NoError
    }
    ///A low threshold error on channel y
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == AWLTF0::Error
    }
}
///Field `AWLTF(0-7)` writer - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
pub type AWLTF_W<'a, REG> = crate::BitWriter<'a, REG, AWLTF0>;
impl<'a, REG> AWLTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No low threshold error
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(AWLTF0::NoError)
    }
    ///A low threshold error on channel y
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(AWLTF0::Error)
    }
}
/**Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWHTF0 {
    ///0: No high threshold error
    NoError = 0,
    ///1: A high threshold error on channel y
    Error = 1,
}
impl From<AWHTF0> for bool {
    #[inline(always)]
    fn from(variant: AWHTF0) -> Self {
        variant as u8 != 0
    }
}
///Field `AWHTF(0-7)` reader - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
pub type AWHTF_R = crate::BitReader<AWHTF0>;
impl AWHTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWHTF0 {
        match self.bits {
            false => AWHTF0::NoError,
            true => AWHTF0::Error,
        }
    }
    ///No high threshold error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AWHTF0::NoError
    }
    ///A high threshold error on channel y
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == AWHTF0::Error
    }
}
///Field `AWHTF(0-7)` writer - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
pub type AWHTF_W<'a, REG> = crate::BitWriter<'a, REG, AWHTF0>;
impl<'a, REG> AWHTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No high threshold error
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(AWHTF0::NoError)
    }
    ///A high threshold error on channel y
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(AWHTF0::Error)
    }
}
impl R {
    ///Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWLTF0` field.</div>
    #[inline(always)]
    pub fn awltf(&self, n: u8) -> AWLTF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AWLTF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf_iter(&self) -> impl Iterator<Item = AWLTF_R> + '_ {
        (0..8).map(move |n| AWLTF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf0(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf1(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf2(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf3(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf4(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf5(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf6(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf7(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWHTF0` field.</div>
    #[inline(always)]
    pub fn awhtf(&self, n: u8) -> AWHTF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AWHTF_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf_iter(&self) -> impl Iterator<Item = AWHTF_R> + '_ {
        (0..8).map(move |n| AWHTF_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    ///Bit 8 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf0(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf1(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf2(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf3(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf4(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf5(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf6(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf7(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWSR")
            .field("awhtf0", &self.awhtf0())
            .field("awhtf1", &self.awhtf1())
            .field("awhtf2", &self.awhtf2())
            .field("awhtf3", &self.awhtf3())
            .field("awhtf4", &self.awhtf4())
            .field("awhtf5", &self.awhtf5())
            .field("awhtf6", &self.awhtf6())
            .field("awhtf7", &self.awhtf7())
            .field("awltf0", &self.awltf0())
            .field("awltf1", &self.awltf1())
            .field("awltf2", &self.awltf2())
            .field("awltf3", &self.awltf3())
            .field("awltf4", &self.awltf4())
            .field("awltf5", &self.awltf5())
            .field("awltf6", &self.awltf6())
            .field("awltf7", &self.awltf7())
            .finish()
    }
}
impl W {
    ///Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWLTF0` field.</div>
    #[inline(always)]
    pub fn awltf(&mut self, n: u8) -> AWLTF_W<'_, AWSRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AWLTF_W::new(self, n)
    }
    ///Bit 0 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf0(&mut self) -> AWLTF_W<'_, AWSRrs> {
        AWLTF_W::new(self, 0)
    }
    ///Bit 1 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf1(&mut self) -> AWLTF_W<'_, AWSRrs> {
        AWLTF_W::new(self, 1)
    }
    ///Bit 2 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf2(&mut self) -> AWLTF_W<'_, AWSRrs> {
        AWLTF_W::new(self, 2)
    }
    ///Bit 3 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf3(&mut self) -> AWLTF_W<'_, AWSRrs> {
        AWLTF_W::new(self, 3)
    }
    ///Bit 4 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf4(&mut self) -> AWLTF_W<'_, AWSRrs> {
        AWLTF_W::new(self, 4)
    }
    ///Bit 5 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf5(&mut self) -> AWLTF_W<'_, AWSRrs> {
        AWLTF_W::new(self, 5)
    }
    ///Bit 6 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf6(&mut self) -> AWLTF_W<'_, AWSRrs> {
        AWLTF_W::new(self, 6)
    }
    ///Bit 7 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf7(&mut self) -> AWLTF_W<'_, AWSRrs> {
        AWLTF_W::new(self, 7)
    }
    ///Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWHTF0` field.</div>
    #[inline(always)]
    pub fn awhtf(&mut self, n: u8) -> AWHTF_W<'_, AWSRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AWHTF_W::new(self, n + 8)
    }
    ///Bit 8 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf0(&mut self) -> AWHTF_W<'_, AWSRrs> {
        AWHTF_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf1(&mut self) -> AWHTF_W<'_, AWSRrs> {
        AWHTF_W::new(self, 9)
    }
    ///Bit 10 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf2(&mut self) -> AWHTF_W<'_, AWSRrs> {
        AWHTF_W::new(self, 10)
    }
    ///Bit 11 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf3(&mut self) -> AWHTF_W<'_, AWSRrs> {
        AWHTF_W::new(self, 11)
    }
    ///Bit 12 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf4(&mut self) -> AWHTF_W<'_, AWSRrs> {
        AWHTF_W::new(self, 12)
    }
    ///Bit 13 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf5(&mut self) -> AWHTF_W<'_, AWSRrs> {
        AWHTF_W::new(self, 13)
    }
    ///Bit 14 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf6(&mut self) -> AWHTF_W<'_, AWSRrs> {
        AWHTF_W::new(self, 14)
    }
    ///Bit 15 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf7(&mut self) -> AWHTF_W<'_, AWSRrs> {
        AWHTF_W::new(self, 15)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`awsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AWSRrs;
impl crate::RegisterSpec for AWSRrs {
    type Ux = u32;
}
///`read()` method returns [`awsr::R`](R) reader structure
impl crate::Readable for AWSRrs {}
///`write(|w| ..)` method takes [`awsr::W`](W) writer structure
impl crate::Writable for AWSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWSR to value 0
impl crate::Resettable for AWSRrs {}
