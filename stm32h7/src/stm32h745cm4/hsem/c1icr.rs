///Register `C1ICR` reader
pub type R = crate::R<C1ICRrs>;
///Register `C1ICR` writer
pub type W = crate::W<C1ICRrs>;
///Field `ISC(0-31)` reader - Interrupt(1) semaphore %s clear
pub type ISC_R = crate::BitReader;
///Field `ISC(0-31)` writer - Interrupt(1) semaphore %s clear
pub type ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Interrupt(1) semaphore (0-31) clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISC0` field.</div>
    #[inline(always)]
    pub fn isc(&self, n: u8) -> ISC_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ISC_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Interrupt(1) semaphore (0-31) clear
    #[inline(always)]
    pub fn isc_iter(&self) -> impl Iterator<Item = ISC_R> + '_ {
        (0..32).map(move |n| ISC_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Interrupt(1) semaphore 0 clear
    #[inline(always)]
    pub fn isc0(&self) -> ISC_R {
        ISC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt(1) semaphore 1 clear
    #[inline(always)]
    pub fn isc1(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt(1) semaphore 2 clear
    #[inline(always)]
    pub fn isc2(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt(1) semaphore 3 clear
    #[inline(always)]
    pub fn isc3(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt(1) semaphore 4 clear
    #[inline(always)]
    pub fn isc4(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt(1) semaphore 5 clear
    #[inline(always)]
    pub fn isc5(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt(1) semaphore 6 clear
    #[inline(always)]
    pub fn isc6(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt(1) semaphore 7 clear
    #[inline(always)]
    pub fn isc7(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt(1) semaphore 8 clear
    #[inline(always)]
    pub fn isc8(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt(1) semaphore 9 clear
    #[inline(always)]
    pub fn isc9(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt(1) semaphore 10 clear
    #[inline(always)]
    pub fn isc10(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt(1) semaphore 11 clear
    #[inline(always)]
    pub fn isc11(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt(1) semaphore 12 clear
    #[inline(always)]
    pub fn isc12(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt(1) semaphore 13 clear
    #[inline(always)]
    pub fn isc13(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt(1) semaphore 14 clear
    #[inline(always)]
    pub fn isc14(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt(1) semaphore 15 clear
    #[inline(always)]
    pub fn isc15(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Interrupt(1) semaphore 16 clear
    #[inline(always)]
    pub fn isc16(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Interrupt(1) semaphore 17 clear
    #[inline(always)]
    pub fn isc17(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Interrupt(1) semaphore 18 clear
    #[inline(always)]
    pub fn isc18(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Interrupt(1) semaphore 19 clear
    #[inline(always)]
    pub fn isc19(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Interrupt(1) semaphore 20 clear
    #[inline(always)]
    pub fn isc20(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Interrupt(1) semaphore 21 clear
    #[inline(always)]
    pub fn isc21(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Interrupt(1) semaphore 22 clear
    #[inline(always)]
    pub fn isc22(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Interrupt(1) semaphore 23 clear
    #[inline(always)]
    pub fn isc23(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Interrupt(1) semaphore 24 clear
    #[inline(always)]
    pub fn isc24(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Interrupt(1) semaphore 25 clear
    #[inline(always)]
    pub fn isc25(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Interrupt(1) semaphore 26 clear
    #[inline(always)]
    pub fn isc26(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Interrupt(1) semaphore 27 clear
    #[inline(always)]
    pub fn isc27(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Interrupt(1) semaphore 28 clear
    #[inline(always)]
    pub fn isc28(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Interrupt(1) semaphore 29 clear
    #[inline(always)]
    pub fn isc29(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Interrupt(1) semaphore 30 clear
    #[inline(always)]
    pub fn isc30(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Interrupt(1) semaphore 31 clear
    #[inline(always)]
    pub fn isc31(&self) -> ISC_R {
        ISC_R::new(((self.bits >> 31) & 1) != 0)
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
    ///Interrupt(1) semaphore (0-31) clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISC0` field.</div>
    #[inline(always)]
    pub fn isc(&mut self, n: u8) -> ISC_W<C1ICRrs> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ISC_W::new(self, n)
    }
    ///Bit 0 - Interrupt(1) semaphore 0 clear
    #[inline(always)]
    pub fn isc0(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 0)
    }
    ///Bit 1 - Interrupt(1) semaphore 1 clear
    #[inline(always)]
    pub fn isc1(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 1)
    }
    ///Bit 2 - Interrupt(1) semaphore 2 clear
    #[inline(always)]
    pub fn isc2(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 2)
    }
    ///Bit 3 - Interrupt(1) semaphore 3 clear
    #[inline(always)]
    pub fn isc3(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 3)
    }
    ///Bit 4 - Interrupt(1) semaphore 4 clear
    #[inline(always)]
    pub fn isc4(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 4)
    }
    ///Bit 5 - Interrupt(1) semaphore 5 clear
    #[inline(always)]
    pub fn isc5(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 5)
    }
    ///Bit 6 - Interrupt(1) semaphore 6 clear
    #[inline(always)]
    pub fn isc6(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 6)
    }
    ///Bit 7 - Interrupt(1) semaphore 7 clear
    #[inline(always)]
    pub fn isc7(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 7)
    }
    ///Bit 8 - Interrupt(1) semaphore 8 clear
    #[inline(always)]
    pub fn isc8(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 8)
    }
    ///Bit 9 - Interrupt(1) semaphore 9 clear
    #[inline(always)]
    pub fn isc9(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 9)
    }
    ///Bit 10 - Interrupt(1) semaphore 10 clear
    #[inline(always)]
    pub fn isc10(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 10)
    }
    ///Bit 11 - Interrupt(1) semaphore 11 clear
    #[inline(always)]
    pub fn isc11(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 11)
    }
    ///Bit 12 - Interrupt(1) semaphore 12 clear
    #[inline(always)]
    pub fn isc12(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 12)
    }
    ///Bit 13 - Interrupt(1) semaphore 13 clear
    #[inline(always)]
    pub fn isc13(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 13)
    }
    ///Bit 14 - Interrupt(1) semaphore 14 clear
    #[inline(always)]
    pub fn isc14(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 14)
    }
    ///Bit 15 - Interrupt(1) semaphore 15 clear
    #[inline(always)]
    pub fn isc15(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 15)
    }
    ///Bit 16 - Interrupt(1) semaphore 16 clear
    #[inline(always)]
    pub fn isc16(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 16)
    }
    ///Bit 17 - Interrupt(1) semaphore 17 clear
    #[inline(always)]
    pub fn isc17(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 17)
    }
    ///Bit 18 - Interrupt(1) semaphore 18 clear
    #[inline(always)]
    pub fn isc18(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 18)
    }
    ///Bit 19 - Interrupt(1) semaphore 19 clear
    #[inline(always)]
    pub fn isc19(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 19)
    }
    ///Bit 20 - Interrupt(1) semaphore 20 clear
    #[inline(always)]
    pub fn isc20(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 20)
    }
    ///Bit 21 - Interrupt(1) semaphore 21 clear
    #[inline(always)]
    pub fn isc21(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 21)
    }
    ///Bit 22 - Interrupt(1) semaphore 22 clear
    #[inline(always)]
    pub fn isc22(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 22)
    }
    ///Bit 23 - Interrupt(1) semaphore 23 clear
    #[inline(always)]
    pub fn isc23(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 23)
    }
    ///Bit 24 - Interrupt(1) semaphore 24 clear
    #[inline(always)]
    pub fn isc24(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 24)
    }
    ///Bit 25 - Interrupt(1) semaphore 25 clear
    #[inline(always)]
    pub fn isc25(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 25)
    }
    ///Bit 26 - Interrupt(1) semaphore 26 clear
    #[inline(always)]
    pub fn isc26(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 26)
    }
    ///Bit 27 - Interrupt(1) semaphore 27 clear
    #[inline(always)]
    pub fn isc27(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 27)
    }
    ///Bit 28 - Interrupt(1) semaphore 28 clear
    #[inline(always)]
    pub fn isc28(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 28)
    }
    ///Bit 29 - Interrupt(1) semaphore 29 clear
    #[inline(always)]
    pub fn isc29(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 29)
    }
    ///Bit 30 - Interrupt(1) semaphore 30 clear
    #[inline(always)]
    pub fn isc30(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 30)
    }
    ///Bit 31 - Interrupt(1) semaphore 31 clear
    #[inline(always)]
    pub fn isc31(&mut self) -> ISC_W<C1ICRrs> {
        ISC_W::new(self, 31)
    }
}
/**HSEM Interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`c1icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#HSEM:C1ICR)*/
pub struct C1ICRrs;
impl crate::RegisterSpec for C1ICRrs {
    type Ux = u32;
}
///`read()` method returns [`c1icr::R`](R) reader structure
impl crate::Readable for C1ICRrs {}
///`write(|w| ..)` method takes [`c1icr::W`](W) writer structure
impl crate::Writable for C1ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C1ICR to value 0
impl crate::Resettable for C1ICRrs {
    const RESET_VALUE: u32 = 0;
}
