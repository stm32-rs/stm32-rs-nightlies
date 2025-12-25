///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
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
///Field `ISC(0-31)` reader - Interrupt semaphore %s clear bit
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
///Field `ISC(0-31)` writer - Interrupt semaphore %s clear bit
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
    ///Interrupt semaphore (0-31) clear bit
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISC0` field.</div>
    #[inline(always)]
    pub fn isc(&self, n: u8) -> ISC_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ISC_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Interrupt semaphore (0-31) clear bit
    #[inline(always)]
    pub fn isc_iter(&self) -> impl Iterator<Item = ISC_R> + '_ {
        (0..32).map(move |n| ISC_R::new(((self.bits >> n) & 1) != 0))
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
    ///Bit 16 - Interrupt semaphore 16 clear bit
    #[inline(always)]
    pub fn isc16(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Interrupt semaphore 17 clear bit
    #[inline(always)]
    pub fn isc17(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Interrupt semaphore 18 clear bit
    #[inline(always)]
    pub fn isc18(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Interrupt semaphore 19 clear bit
    #[inline(always)]
    pub fn isc19(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Interrupt semaphore 20 clear bit
    #[inline(always)]
    pub fn isc20(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Interrupt semaphore 21 clear bit
    #[inline(always)]
    pub fn isc21(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Interrupt semaphore 22 clear bit
    #[inline(always)]
    pub fn isc22(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Interrupt semaphore 23 clear bit
    #[inline(always)]
    pub fn isc23(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Interrupt semaphore 24 clear bit
    #[inline(always)]
    pub fn isc24(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Interrupt semaphore 25 clear bit
    #[inline(always)]
    pub fn isc25(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Interrupt semaphore 26 clear bit
    #[inline(always)]
    pub fn isc26(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Interrupt semaphore 27 clear bit
    #[inline(always)]
    pub fn isc27(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Interrupt semaphore 28 clear bit
    #[inline(always)]
    pub fn isc28(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Interrupt semaphore 29 clear bit
    #[inline(always)]
    pub fn isc29(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Interrupt semaphore 30 clear bit
    #[inline(always)]
    pub fn isc30(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Interrupt semaphore 31 clear bit
    #[inline(always)]
    pub fn isc31(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
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
            .field("isc16", &self.isc16())
            .field("isc17", &self.isc17())
            .field("isc18", &self.isc18())
            .field("isc19", &self.isc19())
            .field("isc20", &self.isc20())
            .field("isc21", &self.isc21())
            .field("isc22", &self.isc22())
            .field("isc23", &self.isc23())
            .field("isc24", &self.isc24())
            .field("isc25", &self.isc25())
            .field("isc26", &self.isc26())
            .field("isc27", &self.isc27())
            .field("isc28", &self.isc28())
            .field("isc29", &self.isc29())
            .field("isc30", &self.isc30())
            .field("isc31", &self.isc31())
            .finish()
    }
}
impl W {
    ///Interrupt semaphore (0-31) clear bit
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISC0` field.</div>
    #[inline(always)]
    pub fn isc(&mut self, n: u8) -> ISC_W<'_, ICRrs> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ISC_W::new(self, n)
    }
    ///Bit 0 - Interrupt semaphore 0 clear bit
    #[inline(always)]
    pub fn isc0(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 0)
    }
    ///Bit 1 - Interrupt semaphore 1 clear bit
    #[inline(always)]
    pub fn isc1(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 1)
    }
    ///Bit 2 - Interrupt semaphore 2 clear bit
    #[inline(always)]
    pub fn isc2(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 2)
    }
    ///Bit 3 - Interrupt semaphore 3 clear bit
    #[inline(always)]
    pub fn isc3(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 3)
    }
    ///Bit 4 - Interrupt semaphore 4 clear bit
    #[inline(always)]
    pub fn isc4(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 4)
    }
    ///Bit 5 - Interrupt semaphore 5 clear bit
    #[inline(always)]
    pub fn isc5(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 5)
    }
    ///Bit 6 - Interrupt semaphore 6 clear bit
    #[inline(always)]
    pub fn isc6(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 6)
    }
    ///Bit 7 - Interrupt semaphore 7 clear bit
    #[inline(always)]
    pub fn isc7(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 7)
    }
    ///Bit 8 - Interrupt semaphore 8 clear bit
    #[inline(always)]
    pub fn isc8(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 8)
    }
    ///Bit 9 - Interrupt semaphore 9 clear bit
    #[inline(always)]
    pub fn isc9(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 9)
    }
    ///Bit 10 - Interrupt semaphore 10 clear bit
    #[inline(always)]
    pub fn isc10(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 10)
    }
    ///Bit 11 - Interrupt semaphore 11 clear bit
    #[inline(always)]
    pub fn isc11(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 11)
    }
    ///Bit 12 - Interrupt semaphore 12 clear bit
    #[inline(always)]
    pub fn isc12(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 12)
    }
    ///Bit 13 - Interrupt semaphore 13 clear bit
    #[inline(always)]
    pub fn isc13(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 13)
    }
    ///Bit 14 - Interrupt semaphore 14 clear bit
    #[inline(always)]
    pub fn isc14(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 14)
    }
    ///Bit 15 - Interrupt semaphore 15 clear bit
    #[inline(always)]
    pub fn isc15(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 15)
    }
    ///Bit 16 - Interrupt semaphore 16 clear bit
    #[inline(always)]
    pub fn isc16(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 16)
    }
    ///Bit 17 - Interrupt semaphore 17 clear bit
    #[inline(always)]
    pub fn isc17(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 17)
    }
    ///Bit 18 - Interrupt semaphore 18 clear bit
    #[inline(always)]
    pub fn isc18(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 18)
    }
    ///Bit 19 - Interrupt semaphore 19 clear bit
    #[inline(always)]
    pub fn isc19(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 19)
    }
    ///Bit 20 - Interrupt semaphore 20 clear bit
    #[inline(always)]
    pub fn isc20(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 20)
    }
    ///Bit 21 - Interrupt semaphore 21 clear bit
    #[inline(always)]
    pub fn isc21(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 21)
    }
    ///Bit 22 - Interrupt semaphore 22 clear bit
    #[inline(always)]
    pub fn isc22(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 22)
    }
    ///Bit 23 - Interrupt semaphore 23 clear bit
    #[inline(always)]
    pub fn isc23(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 23)
    }
    ///Bit 24 - Interrupt semaphore 24 clear bit
    #[inline(always)]
    pub fn isc24(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 24)
    }
    ///Bit 25 - Interrupt semaphore 25 clear bit
    #[inline(always)]
    pub fn isc25(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 25)
    }
    ///Bit 26 - Interrupt semaphore 26 clear bit
    #[inline(always)]
    pub fn isc26(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 26)
    }
    ///Bit 27 - Interrupt semaphore 27 clear bit
    #[inline(always)]
    pub fn isc27(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 27)
    }
    ///Bit 28 - Interrupt semaphore 28 clear bit
    #[inline(always)]
    pub fn isc28(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 28)
    }
    ///Bit 29 - Interrupt semaphore 29 clear bit
    #[inline(always)]
    pub fn isc29(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 29)
    }
    ///Bit 30 - Interrupt semaphore 30 clear bit
    #[inline(always)]
    pub fn isc30(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 30)
    }
    ///Bit 31 - Interrupt semaphore 31 clear bit
    #[inline(always)]
    pub fn isc31(&mut self) -> ISC_W<'_, ICRrs> {
        ISC_W::new(self, 31)
    }
}
/**HSEM Interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HSEM:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
