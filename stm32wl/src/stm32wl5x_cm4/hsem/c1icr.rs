///Register `C1ICR` reader
pub type R = crate::R<C1ICRrs>;
///Register `C1ICR` writer
pub type W = crate::W<C1ICRrs>;
/**Interrupt semaphore %s clear bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISC0R {
    ///0: Always reads 0
    NoEffect = 0,
}
impl From<ISC0R> for bool {
    #[inline(always)]
    fn from(variant: ISC0R) -> Self {
        variant as u8 != 0
    }
}
///Field `ISC(0-15)` reader - Interrupt semaphore %s clear bit
pub type ISC_R = crate::BitReader<ISC0R>;
impl ISC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ISC0R> {
        match self.bits {
            false => Some(ISC0R::NoEffect),
            _ => None,
        }
    }
    ///Always reads 0
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ISC0R::NoEffect
    }
}
/**Interrupt semaphore %s clear bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISC0W {
    ///0: Interrupt semaphore x status ISFx and masked status MISFx not affected
    NoEffect = 0,
    ///1: Interrupt semaphore x status ISFx and masked status MISFx cleared
    Clear = 1,
}
impl From<ISC0W> for bool {
    #[inline(always)]
    fn from(variant: ISC0W) -> Self {
        variant as u8 != 0
    }
}
///Field `ISC(0-15)` writer - Interrupt semaphore %s clear bit
pub type ISC_W<'a, REG> = crate::BitWriter<'a, REG, ISC0W>;
impl<'a, REG> ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt semaphore x status ISFx and masked status MISFx not affected
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0W::NoEffect)
    }
    ///Interrupt semaphore x status ISFx and masked status MISFx cleared
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0W::Clear)
    }
}
impl R {
    ///Interrupt semaphore (0-15) clear bit
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISC0` field.</div>
    #[inline(always)]
    pub fn isc(&self, n: u8) -> ISC_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ISC_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Interrupt semaphore (0-15) clear bit
    #[inline(always)]
    pub fn isc_iter(&self) -> impl Iterator<Item = ISC_R> + '_ {
        (0..16).map(move |n| ISC_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Interrupt semaphore 0 clear bit
    #[inline(always)]
    pub fn isc0(&self) -> ISC_R {
        ISC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt semaphore 1 clear bit
    #[inline(always)]
    pub fn isc1(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt semaphore 2 clear bit
    #[inline(always)]
    pub fn isc2(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt semaphore 3 clear bit
    #[inline(always)]
    pub fn isc3(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt semaphore 4 clear bit
    #[inline(always)]
    pub fn isc4(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt semaphore 5 clear bit
    #[inline(always)]
    pub fn isc5(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt semaphore 6 clear bit
    #[inline(always)]
    pub fn isc6(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt semaphore 7 clear bit
    #[inline(always)]
    pub fn isc7(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt semaphore 8 clear bit
    #[inline(always)]
    pub fn isc8(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt semaphore 9 clear bit
    #[inline(always)]
    pub fn isc9(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt semaphore 10 clear bit
    #[inline(always)]
    pub fn isc10(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt semaphore 11 clear bit
    #[inline(always)]
    pub fn isc11(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt semaphore 12 clear bit
    #[inline(always)]
    pub fn isc12(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt semaphore 13 clear bit
    #[inline(always)]
    pub fn isc13(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt semaphore 14 clear bit
    #[inline(always)]
    pub fn isc14(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt semaphore 15 clear bit
    #[inline(always)]
    pub fn isc15(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1ICR")
            .field("isc0", &self.isc0())
            .field("isc1", &self.isc1())
            .field("isc2", &self.isc2())
            .field("isc3", &self.isc3())
            .field("isc4", &self.isc4())
            .field("isc5", &self.isc5())
            .field("isc6", &self.isc6())
            .field("isc7", &self.isc7())
            .field("isc8", &self.isc8())
            .field("isc9", &self.isc9())
            .field("isc10", &self.isc10())
            .field("isc11", &self.isc11())
            .field("isc12", &self.isc12())
            .field("isc13", &self.isc13())
            .field("isc14", &self.isc14())
            .field("isc15", &self.isc15())
            .finish()
    }
}
impl W {
    ///Interrupt semaphore (0-15) clear bit
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISC0` field.</div>
    #[inline(always)]
    pub fn isc(&mut self, n: u8) -> ISC_W<'_, C1ICRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ISC_W::new(self, n)
    }
    ///Bit 0 - Interrupt semaphore 0 clear bit
    #[inline(always)]
    pub fn isc0(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 0)
    }
    ///Bit 1 - Interrupt semaphore 1 clear bit
    #[inline(always)]
    pub fn isc1(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 1)
    }
    ///Bit 2 - Interrupt semaphore 2 clear bit
    #[inline(always)]
    pub fn isc2(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 2)
    }
    ///Bit 3 - Interrupt semaphore 3 clear bit
    #[inline(always)]
    pub fn isc3(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 3)
    }
    ///Bit 4 - Interrupt semaphore 4 clear bit
    #[inline(always)]
    pub fn isc4(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 4)
    }
    ///Bit 5 - Interrupt semaphore 5 clear bit
    #[inline(always)]
    pub fn isc5(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 5)
    }
    ///Bit 6 - Interrupt semaphore 6 clear bit
    #[inline(always)]
    pub fn isc6(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 6)
    }
    ///Bit 7 - Interrupt semaphore 7 clear bit
    #[inline(always)]
    pub fn isc7(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 7)
    }
    ///Bit 8 - Interrupt semaphore 8 clear bit
    #[inline(always)]
    pub fn isc8(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 8)
    }
    ///Bit 9 - Interrupt semaphore 9 clear bit
    #[inline(always)]
    pub fn isc9(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 9)
    }
    ///Bit 10 - Interrupt semaphore 10 clear bit
    #[inline(always)]
    pub fn isc10(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 10)
    }
    ///Bit 11 - Interrupt semaphore 11 clear bit
    #[inline(always)]
    pub fn isc11(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 11)
    }
    ///Bit 12 - Interrupt semaphore 12 clear bit
    #[inline(always)]
    pub fn isc12(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 12)
    }
    ///Bit 13 - Interrupt semaphore 13 clear bit
    #[inline(always)]
    pub fn isc13(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 13)
    }
    ///Bit 14 - Interrupt semaphore 14 clear bit
    #[inline(always)]
    pub fn isc14(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 14)
    }
    ///Bit 15 - Interrupt semaphore 15 clear bit
    #[inline(always)]
    pub fn isc15(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 15)
    }
}
/**HSEM Interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`c1icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#HSEM:C1ICR)*/
pub struct C1ICRrs;
impl crate::RegisterSpec for C1ICRrs {
    type Ux = u32;
}
///`read()` method returns [`c1icr::R`](R) reader structure
impl crate::Readable for C1ICRrs {}
///`write(|w| ..)` method takes [`c1icr::W`](W) writer structure
impl crate::Writable for C1ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1ICR to value 0
impl crate::Resettable for C1ICRrs {}
