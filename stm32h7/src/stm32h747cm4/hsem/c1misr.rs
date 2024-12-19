///Register `C1MISR` reader
pub type R = crate::R<C1MISRrs>;
///Field `ISEM(0-31)` reader - masked interrupt(1) semaphore %s status bit
pub type ISEM_R = crate::BitReader;
impl R {
    ///masked interrupt(1) semaphore (0-31) status bit
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISEM0` field.</div>
    #[inline(always)]
    pub fn isem(&self, n: u8) -> ISEM_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ISEM_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///masked interrupt(1) semaphore (0-31) status bit
    #[inline(always)]
    pub fn isem_iter(&self) -> impl Iterator<Item = ISEM_R> + '_ {
        (0..32).map(move |n| ISEM_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - masked interrupt(1) semaphore 0 status bit
    #[inline(always)]
    pub fn isem0(&self) -> ISEM_R {
        ISEM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - masked interrupt(1) semaphore 1 status bit
    #[inline(always)]
    pub fn isem1(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - masked interrupt(1) semaphore 2 status bit
    #[inline(always)]
    pub fn isem2(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - masked interrupt(1) semaphore 3 status bit
    #[inline(always)]
    pub fn isem3(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - masked interrupt(1) semaphore 4 status bit
    #[inline(always)]
    pub fn isem4(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - masked interrupt(1) semaphore 5 status bit
    #[inline(always)]
    pub fn isem5(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - masked interrupt(1) semaphore 6 status bit
    #[inline(always)]
    pub fn isem6(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - masked interrupt(1) semaphore 7 status bit
    #[inline(always)]
    pub fn isem7(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - masked interrupt(1) semaphore 8 status bit
    #[inline(always)]
    pub fn isem8(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - masked interrupt(1) semaphore 9 status bit
    #[inline(always)]
    pub fn isem9(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - masked interrupt(1) semaphore 10 status bit
    #[inline(always)]
    pub fn isem10(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - masked interrupt(1) semaphore 11 status bit
    #[inline(always)]
    pub fn isem11(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - masked interrupt(1) semaphore 12 status bit
    #[inline(always)]
    pub fn isem12(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - masked interrupt(1) semaphore 13 status bit
    #[inline(always)]
    pub fn isem13(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - masked interrupt(1) semaphore 14 status bit
    #[inline(always)]
    pub fn isem14(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - masked interrupt(1) semaphore 15 status bit
    #[inline(always)]
    pub fn isem15(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - masked interrupt(1) semaphore 16 status bit
    #[inline(always)]
    pub fn isem16(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - masked interrupt(1) semaphore 17 status bit
    #[inline(always)]
    pub fn isem17(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - masked interrupt(1) semaphore 18 status bit
    #[inline(always)]
    pub fn isem18(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - masked interrupt(1) semaphore 19 status bit
    #[inline(always)]
    pub fn isem19(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - masked interrupt(1) semaphore 20 status bit
    #[inline(always)]
    pub fn isem20(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - masked interrupt(1) semaphore 21 status bit
    #[inline(always)]
    pub fn isem21(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - masked interrupt(1) semaphore 22 status bit
    #[inline(always)]
    pub fn isem22(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - masked interrupt(1) semaphore 23 status bit
    #[inline(always)]
    pub fn isem23(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - masked interrupt(1) semaphore 24 status bit
    #[inline(always)]
    pub fn isem24(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - masked interrupt(1) semaphore 25 status bit
    #[inline(always)]
    pub fn isem25(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - masked interrupt(1) semaphore 26 status bit
    #[inline(always)]
    pub fn isem26(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - masked interrupt(1) semaphore 27 status bit
    #[inline(always)]
    pub fn isem27(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - masked interrupt(1) semaphore 28 status bit
    #[inline(always)]
    pub fn isem28(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - masked interrupt(1) semaphore 29 status bit
    #[inline(always)]
    pub fn isem29(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - masked interrupt(1) semaphore 30 status bit
    #[inline(always)]
    pub fn isem30(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - masked interrupt(1) semaphore 31 status bit
    #[inline(always)]
    pub fn isem31(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1MISR")
            .field("isem0", &self.isem0())
            .field("isem1", &self.isem1())
            .field("isem2", &self.isem2())
            .field("isem3", &self.isem3())
            .field("isem4", &self.isem4())
            .field("isem5", &self.isem5())
            .field("isem6", &self.isem6())
            .field("isem7", &self.isem7())
            .field("isem8", &self.isem8())
            .field("isem9", &self.isem9())
            .field("isem10", &self.isem10())
            .field("isem11", &self.isem11())
            .field("isem12", &self.isem12())
            .field("isem13", &self.isem13())
            .field("isem14", &self.isem14())
            .field("isem15", &self.isem15())
            .field("isem16", &self.isem16())
            .field("isem17", &self.isem17())
            .field("isem18", &self.isem18())
            .field("isem19", &self.isem19())
            .field("isem20", &self.isem20())
            .field("isem21", &self.isem21())
            .field("isem22", &self.isem22())
            .field("isem23", &self.isem23())
            .field("isem24", &self.isem24())
            .field("isem25", &self.isem25())
            .field("isem26", &self.isem26())
            .field("isem27", &self.isem27())
            .field("isem28", &self.isem28())
            .field("isem29", &self.isem29())
            .field("isem30", &self.isem30())
            .field("isem31", &self.isem31())
            .finish()
    }
}
/**HSEM Masked interrupt status of Line 1

You can [`read`](crate::Reg::read) this register and get [`c1misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#HSEM:C1MISR)*/
pub struct C1MISRrs;
impl crate::RegisterSpec for C1MISRrs {
    type Ux = u32;
}
///`read()` method returns [`c1misr::R`](R) reader structure
impl crate::Readable for C1MISRrs {}
///`reset()` method sets C1MISR to value 0
impl crate::Resettable for C1MISRrs {
    const RESET_VALUE: u32 = 0;
}
